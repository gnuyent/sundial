# -*- coding: utf-8 -*-
import re

# TODO: Update docstrings to be in numpy format.


def clean(bloat, standard_length):
    """Clean up unnecessary data from course information.

    Data cannot be cleanly parsed due to whitespace, unnecessary characters, etc. This function removes unnecessary noise from strings. # noqa: E501

    Parameters
    ----------
    bloat : List[str]
        List of "dirty" strings to cleanup.
    standard_length : int
        Number of meetings so all lists are the same length.

    Returns
    -------
    List[str]
        Cleaned list of strings.
    """
    for i, item in enumerate(bloat):  # Remove all angled brackets and text within them
        bloat[i] = re.sub(r"<(.*?)>", "", bloat[i])
    bloat = [item.strip() for item in bloat]  # Remove white space
    if len(bloat) > standard_length:
        bloat = list(filter(None, bloat))  # Replace empty indices
    if standard_length > len(bloat):
        while len(bloat) < standard_length:
            bloat.append("")
    return bloat


def parse_meetings(sel):
    """Parse meetings from a scrapy selector.

    Parameters
    ----------
    sel
        Selector to generate meetings over.
    """
    keys = ["types", "times", "days", "locations", "instructors"]
    meetings = {key: [] for key in keys}
    # Some courses have courses that link to other courses, therefore we filter explicitly here. # noqa: E501
    meeting_count = len(sel.xpath('.//div[@class="sectionFieldType column"]'))
    for parameter in ["Type", "Time", "Day", "Location", "Instructor"]:
        search_string = f'.//div[@class="sectionField{parameter} column"]//text()'
        meeting_item = sel.xpath(search_string)
        # Create an empty string if column does not exist and remove html formatting.
        try:
            meeting_item = clean(meeting_item.getall(), meeting_count)
        except KeyError:
            meeting_item = clean(meeting_item.getall(), meeting_count)

        dict_key = parameter.lower() + "s"
        for value in meeting_item:
            meetings[dict_key].append(value)

    return meetings


def parse_footnotes(sel):
    """Parse footnotes from a scrapy selector.

    Parameters
    ----------
    sel :
        Selector to generate footnotes over.
    """
    footnotes = {}
    footnote_codes = clean(
        sel.xpath('.//div[@class="footnoteCode column"]').getall(), True
    )
    footnote_details = clean(
        sel.xpath('.//div[@class="footnoteDetails column"]').getall(), True
    )
    for idx, code in enumerate(footnote_codes):
        footnotes[footnote_codes[idx]] = footnote_details[idx]
    return footnotes
