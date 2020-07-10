from dataclasses import dataclass

from sundial.scheduler.period import DateTime


@dataclass
class Meeting:
    """
    Store meeting information for Course.

    Parameters
    ----------
    date : DateTime
        Date of the meeting.
    meeting_id : str
        Meeting ID from the database.
    """

    date: DateTime = DateTime()
    meeting_id: str = ""
