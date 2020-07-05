# -*- coding: utf-8 -*-
import re


def clean(bloat, standard_length):
    """
    Cleans up unnecessary data from course information.
    :param standard_length: Number of meetings so all lists are the same length
    :param bloat: Array of "dirty" strings to cleanup
    :return: Cleaned array of Strings.
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
    keys = ["types", "times", "days", "locations", "instructors"]
    meetings = {key: [] for key in keys}
    meeting_count = len(sel.xpath('.//div[@class="sectionFieldType column"]'))
    # Filter by table to reduce extraneous information from classes that are HEADS of sections
    for parameter in ["Type", "Time", "Day", "Location", "Instructor"]:
        search_string = f'.//div[@class="sectionField{parameter} column"]//text()'
        meeting_item = sel.xpath(search_string)
        # Create an empty string if column does not exist and remove html formatting
        try:
            meeting_item = clean(meeting_item.getall(), meeting_count)
        except KeyError:
            meeting_item = clean(meeting_item.getall(), meeting_count)

        dict_key = parameter.lower() + "s"
        for value in meeting_item:
            meetings[dict_key].append(value)

    return meetings


def parse_footnotes(sel):
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

