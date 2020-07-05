from hourglass.datetime import DateTime, Day


class Meeting:
    def __init__(
        self,
        date: DateTime = DateTime(),
        instructor: str = "",
        location: str = "",
        meeting_id: str = "",
        meeting_type: str = "",
    ):
        self.date = date
        self.instructor = instructor
        self.location = location
        self.meeting_id = meeting_id
        self.meeting_type = meeting_type
