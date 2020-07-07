from flask import Flask, jsonify
from flask_restful import Api, Resource
from flask_sqlalchemy import SQLAlchemy

from resources.models.course import Course

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///../classes.db"
app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False
api = Api(app)
db = SQLAlchemy(app)


class CourseAPI(Resource):
    def get(self, schedule_num):
        query = Course.query.filter_by(schedule_num=schedule_num).all()
        json = [field.serialize for field in query]
        return jsonify(json_list=json)


class SubjectAPI(Resource):
    def get(self, subject):
        search = f"{subject}-%"
        query = Course.query.filter(Course.course.like(search)).all()
        json = [field.serialize for field in query]
        return jsonify(json_list=json)


api.add_resource(CourseAPI, "/api/course/<string:schedule_num>")
api.add_resource(SubjectAPI, "/api/subject/<string:subject>")

if __name__ == "__main__":
    app.run(debug=True)
