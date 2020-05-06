import re
import itertools
from datetime import datetime, timedelta

from random import choice
from chatterbot.logic import LogicAdapter
from chatterbot.comparisons import LevenshteinDistance
from chatterbot.conversation import Statement

class WelfareAdapter(LogicAdapter):
    def __init__(self, chatbot, **kwargs):
        super().__init__(chatbot, **kwargs)

    def can_process(self, statement):
        phrases = [' '.join(x) for x in itertools.product([
            "ile do wyplaty",
            "kiedy wyplata",
            "kiedy bedzie wyplata",
            "wyplata",
            "ile dni do",
            "wyplata"
        ], [
            "500", "500+", "500 plus", "piecet plust", "piecet"
        ])]
        best = max([LevenshteinDistance().compare(statement, Statement(p)) for p in phrases])
        return best > 0.8

    def process(self, input_statement, additional_response_selection_parameters):
        current = datetime.date(datetime.now())

        day, month, year = current.day, current.month, current.year
        target_day = 19

        if day == target_day:
            result = Statement("Jest %s. Wypłata jest dzisiaj!" % current)
            result.confidence = 1.0
            return result

        target_month = month+1 if day > target_day else month
        target_year = year+1 if target_month > 12 else year
        target_month = target_month%12

        target = datetime.date(datetime(target_year, target_month, target_day))
        diff = target-current

        result = Statement("Mamy %s. Do wypłaty 500+ zostało %s dni." % (current, diff.days))
        result.confidence = 1.0
        return result
