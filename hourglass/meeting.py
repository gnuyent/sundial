from hourglass.period import DateTime


class Meeting:
    """
    Store meeting information for Course.

    Parameters
    ----------
    date : DateTime
        Date of the meeting.
    instructor : str
        Name of the instructor.
    location : str
        Location of the meeting.
    meeting_id : str
        Meeting ID from the database.
    meeting_type : str
        Type of meeting.
    """

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
