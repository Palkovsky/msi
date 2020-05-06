import re
import requests
import itertools

from random import choice
from chatterbot.logic import LogicAdapter
from chatterbot.comparisons import LevenshteinDistance
from chatterbot.conversation import Statement

class CurrencyAdapter(LogicAdapter):
    prefixes = [
        "kurs",
        "cena",
        "kosztuje",
        "ile kosztuje",
        "po ile",
        "wartosc"
    ]
    currencies = [
        (["dolar amerykanski", "dolara amerykanskiego", "dolara", "dolar", "usd"], "usd"),
        (["euro", "eur"], "eur"),
        (["frank szwajcarski", "franka szwajcarskiego", "franka", "frank", "chf"], "chf"),
    ]
    currency_lookup = {s:val for (syn, val) in currencies for s in syn}
    currencies, _ = list(zip(*currencies))
    currencies = itertools.chain(*currencies)
    regex = re.compile("(%s).*(%s)(.*|$)" % ('|'.join(prefixes), '|'.join(currencies)))

    def __init__(self, chatbot, **kwargs):
        super().__init__(chatbot, **kwargs)

    def can_process(self, statement):
        results = re.findall(CurrencyAdapter.regex, statement.text)
        if len(results) > 0:
            currency = results[0][1]
            statement.text = CurrencyAdapter.currency_lookup.get(currency)
        return statement.text in CurrencyAdapter.currency_lookup

    def process(self, input_statement, additional_response_selection_parameters):
        currency = input_statement.text.upper()
        url = "https://api.exchangeratesapi.io/latest?base=%s&symbols=PLN" % currency
        text = "Nie udało się połączyć z zewnątrzną usługą aby pobrać kurs %s" % currency
        try:
            resp = requests.get(url=url)
            data = resp.json()
            price = data["rates"]["PLN"]
            text = "Kurs %s to %f" % (currency, price)
        except (Exception):
            print(e)

        result = Statement(text)
        result.confidence = 1.0
        return result
