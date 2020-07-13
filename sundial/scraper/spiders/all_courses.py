# -*- coding: utf-8 -*-
import scrapy
from scrapy.linkextractors import LinkExtractor
from scrapy.selector import Selector

from sundial.scraper.items import CourseItem
from sundial.scraper.utilities import clean, parse_footnotes, parse_meetings


class AllCoursesSpider(scrapy.Spider):
    """Generate initial database with complete data.

    AllCoursesSpider builds the database by executing a complete scrape over the scheduling website. It determines the period during the year by the parameter it is passed. # noqa: E501

    Parameters
    ----------
    year: int
        The year to scrape from formatted in <YYYY>.
        (default: 2020)
    season : int
        The season within the year to scrape from. 2 = Spring. 3 = Summer. 4 = Fall.
        (default: 4)
    """

    name = "allcourses"
    allowed_domains = ["sunspot.sdsu.edu"]

    def __init__(self, period: int = 2020, season: int = 4, **kwargs):
        if not 2 <= season <= 4:
            raise ValueError(f"{season} is not between 2 and 4.")
        self.start_urls = [
            f"https://sunspot.sdsu.edu/schedule/search?mode=browse_by_subject&category=browse_by_subject"  # noqa: E501
            f"&period={period}"
        ]
        self.period = period
        super().__init__(**kwargs)

    def process_value(self, value):
        """Retrieve the correct link by appending the period.

        Parameters
        ----------
        value
            Link to append to.
        """
        return value + f"&period={self.period}"

    def parse(self, response):
        """Scrape links to each subject.

        Parameters
        ----------
        response
            All subjects list to generate individual subject links over.
        """
        subject_links = LinkExtractor(
            process_value=self.process_value,
            restrict_xpaths='.//div[@id="browseContainer"]/ul',
        ).extract_links(response)
        yield from response.follow_all(subject_links, self.scrape_course_links)

    def scrape_course_links(self, response):
        """Scrape links to each course.

        Parameters
        ----------
        response
            All courses list to generate individual course links over.
        """
        course_links = LinkExtractor(
            restrict_xpaths='.//div[@class="sectionFieldCourse column"]/a'
        ).extract_links(
            response
        )  # Does not need process_value option because it is automatically included
        yield from response.follow_all(course_links, self.parse_course_information)

    def parse_course_information(self, response):
        """Parse course information into scrapy item.

        Parameters
        ----------
        response
            Course response to parse over.
        """
        sel = Selector(response)
        course_labels = sel.xpath('.//td[@class="sectionDetailLabel"]/text()').getall()
        # Format labels to be same format as CourseItem
        course_labels = [
            index.replace(" ", "_").replace("#", "num").lower()
            for index in course_labels
        ]
        # Remove special-case labels
        course_labels = [
            label for label in course_labels if label not in ("meetings", "footnotes")
        ]
        course_text = clean(
            sel.xpath('.//td[@class="sectionDetailContent"]/text()').getall(), True
        )

        item = CourseItem()

        # Match labels and text
        for idx, descriptor in enumerate(course_labels):
            item[course_labels[idx]] = course_text[idx]

        # Check for upper division courses with no schedule number
        if item["schedule_num"] == "*****":
            item["schedule_num"] = 0

        item["period"] = sel.xpath(".//option[@selected]/@value")[1].get()
        item["seats_available"] = item["seats"].split("/")[0]
        item["seats_total"] = item["seats"].split("/")[1]
        item["url"] = response.url
        item["id"] = (
            item["period"]
            + "-"
            + item["course"].replace("-", "").replace(" ", "")
            + "-"
            + item["section"]
            + "-"
            + str(item["schedule_num"])
        )
        item["meetings"] = parse_meetings(sel)
        item["footnotes"] = parse_footnotes(sel)

        # Fill in empty values
        yield item
