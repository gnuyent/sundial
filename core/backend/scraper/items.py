# -*- coding: utf-8 -*-

# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html
import scrapy


class CourseItem(scrapy.Item):
    id = scrapy.Field()
    url = scrapy.Field()
    period = scrapy.Field()
    course = scrapy.Field()
    course_title = scrapy.Field()
    section = scrapy.Field()
    schedule_num = scrapy.Field()
    units = scrapy.Field()
    session = scrapy.Field()
    seats = scrapy.Field()
    seats_available = scrapy.Field()
    seats_total = scrapy.Field()
    full_title = scrapy.Field()
    description = scrapy.Field()  # optional
    prerequisite = scrapy.Field()  # optional
    course_hours = scrapy.Field()  # optional
    statement = scrapy.Field()  # optional
    general_text = scrapy.Field()  # optional
    meetings = scrapy.Field()
    footnotes = scrapy.Field()
