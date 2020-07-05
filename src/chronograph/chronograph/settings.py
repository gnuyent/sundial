BOT_NAME = 'chronograph'

SPIDER_MODULES = ['chronograph.spiders']
NEWSPIDER_MODULE = 'chronograph.spiders'

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
    'chronograph.pipelines.ChronographDatabasePipeline': 100,
}

# Database settings
CONNECTION_STRING = 'sqlite:///classes.db'
