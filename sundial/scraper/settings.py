BOT_NAME = "sundial"

SPIDER_MODULES = ["scraper.spiders"]
NEWSPIDER_MODULE = "scraper.spiders"

# Obey robots.txt rules
ROBOTSTXT_OBEY = False

# Configure maximum concurrent requests performed by Scrapy (default: 16)
CONCURRENT_REQUESTS = 1
"""
Play around with this number. Current value is ~2x slower than default 16.
16 causes data to be incorrectly scraped.
"""

# Configure item pipelines
# See https://docs.scrapy.org/en/latest/topics/item-pipeline.html
ITEM_PIPELINES = {
    "scraper.pipelines.ScraperDatabasePipeline": 100,
}

# Database settings
CONNECTION_STRING = "sqlite:///classes.db"
