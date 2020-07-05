# -*- coding: utf-8 -*-
import scrapy
from scrapy.linkextractors import LinkExtractor
from scrapy.selector import Selector

from core.backend.scraper.items import CourseItem
from core.backend.scraper.utilities import clean, parse_footnotes, parse_meetings


class AllCoursesSpider(scrapy.Spider):
    name = "allcourses"
    allowed_domains = ["sunspot.sdsu.edu"]

    def __init__(self, period="20204", **kwargs):
        """
        Initializes the start url with a given period to scrape.
        :param period: Default is 20204 - Fall 2020. The correct format will be <YYYY><2-4>.
        2 is Spring. 3 is Summer. 4 is Fall.
        """
        semester = int(str(period)[-1])
        if semester < 2 or semester > 4:
            raise ValueError("Invalid period provided.")
        self.start_urls = [
            f"https://sunspot.sdsu.edu/schedule/search?mode=browse_by_subject&category=browse_by_subject"
            f"&period={period}"
        ]
        self.period = period
        super().__init__(**kwargs)

    def process_value(self, value):
        return value + f"&period={self.period}"

    def parse(self, response):
        """
        Scrapes links to each subject
        """
        subject_links = LinkExtractor(
            process_value=self.process_value,
            restrict_xpaths='.//div[@id="browseContainer"]/ul',
        ).extract_links(response)
        yield from response.follow_all(subject_links, self.scrape_course_links)

    def scrape_course_links(self, response):
        """
        Scrapes links to each course
        """
        course_links = LinkExtractor(
            restrict_xpaths='.//div[@class="sectionFieldCourse column"]/a'
        ).extract_links(
            response
        )  # Does not need process_value option because it is automatically included
        yield from response.follow_all(course_links, self.parse_course_information)

    def parse_course_information(self, response):
        """
        Parses course information
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

