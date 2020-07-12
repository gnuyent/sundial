from typing import List

from fastapi import APIRouter

router = APIRouter()


@router.post("/api/schedule/")
def generate_schedules():
    schedules = List[int]
    pass
