from fastapi import FastAPI

from sundial import models
from sundial.database import engine
from sundial.routers import course, schedule, subject

models.Base.metadata.create_all(bind=engine)

app = FastAPI()


app.include_router(course.router, prefix="/api/course")
app.include_router(schedule.router, prefix="/api/schedule")
app.include_router(subject.router, prefix="/api/subject")
