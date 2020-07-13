# -*- coding: utf-8 -*-

# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html
from sqlalchemy.exc import IntegrityError

from sundial.database import SessionLocal, create_table, engine
from sundial.models import Course, Footnote, Meeting


class ScraperDatabasePipeline:
    """Pipeline for inserting scrapy items into the database."""

    def __init__(self):
        """Connect to database and create tables."""
        create_table(engine)
        self.session = SessionLocal

    def process_item(self, item, spider):
        """Process courses, meetings, and footnotes and save into the database. Called once per item."""  # noqa: D400, E501
        session = self.session()
        course = Course()
        course.id = item["id"]
        course.url = item["url"]
        course.period = item["period"]
        course.course = item["course"]
        course.course_title = item["course_title"]
        course.section = item["section"]
        course.schedule_num = item["schedule_num"]
        course.units = item["units"]
        course.session = item["session"]
        course.seats_available = item["seats_available"]
        course.seats_total = item["seats_total"]
        course.full_title = item["full_title"]

        try:
            course.description = item["description"]
        except KeyError:
            course.description = None
        try:
            course.course_hours = item["course_hours"]
        except KeyError:
            course.course_hours = None
        try:
            course.prerequisite = item["prerequisite"]
        except KeyError:
            course.prerequisite = None
        try:
            course.statement = item["statement"]
        except KeyError:
            course.statement = None
        try:
            course.general_text = item["general_text"]
        except KeyError:
            course.general_text = None

        # Insert meetings
        meeting_counter = 1
        for idx, individual_meeting in enumerate(item["meetings"]["types"]):
            meeting = Meeting()
            meeting.meeting_id = item["id"] + "-" + str(meeting_counter)
            meeting.meeting_type = item["meetings"]["types"][idx]
            meeting.hours = item["meetings"]["times"][idx]
            meeting.days = item["meetings"]["days"][idx]
            meeting.location = item["meetings"]["locations"][idx]
            meeting.instructor = item["meetings"]["instructors"][idx]
            meeting_counter += 1
            course.meetings.append(meeting)

        # Insert footnotes
        for code, text in item["footnotes"].items():
            footnote = Footnote()
            footnote.footnote_id = item["id"] + "-" + code
            footnote.code = code
            footnote.text = text
            course.footnotes.append(footnote)

        try:
            session.add(course)
            session.commit()
        except IntegrityError:  # In case item already exists in table
            session.rollback()
        finally:
            session.close()

        return item
