from setuptools import find_packages, setup

setup(
    name="sundial",
    description="A class scheduling wrapper for San Diego State University.",
    version="0.1.3",
    packages=find_packages(),
    install_requires=["SQLAlchemy", "Scrapy", "fastapi", "uvicorn"],
)
