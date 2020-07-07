from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()


class Course(db.Model):
    id = db.Column("id", db.String(20), primary_key=True)
    url = db.Column("url", db.Text)
    period = db.Column("period", db.Integer)
    course = db.Column("course", db.Text)
    course_title = db.Column("course_title", db.Text)
    section = db.Column("section", db.Integer)
    schedule_num = db.Column("schedule_num", db.Integer)
    units = db.Column("units", db.Float)
    session = db.Column("session", db.Text)
    seats_available = db.Column("seats_available", db.Integer)
    seats_total = db.Column("seats_total", db.Integer)
    full_title = db.Column("full_title", db.Text)
    description = db.Column("description", db.Text)
    prerequisite = db.Column("prerequisite", db.Text)
    course_hours = db.Column("course_hours", db.Text)
    statement = db.Column("statement", db.Text)
    general_text = db.Column("general_text", db.Text)

    @property
    def serialize(self):
        """Return Course in easily serializable format."""
        return {
            "id": self.id,
            "url": self.url,
            "period": self.period,
            "course": self.course,
            "course_title": self.course_title,
            "section": self.section,
            "schedule_num": self.schedule_num,
            "units": self.units,
            "session": self.session,
            "seats_available": self.seats_available,
            "seats_total": self.seats_total,
            "full_title": self.full_title,
            "description": self.description,
            "prerequisite": self.prerequisite,
            "course_hours": self.course_hours,
            "statement": self.statement,
            "general_text": self.general_text,
        }
