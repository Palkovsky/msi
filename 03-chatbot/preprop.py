import string
import unidecode

# some helper preprocessors
def lowercased(statement):
    statement.text = statement.text.lower()
    return statement


# filter out diacritics
def filter_punctuation(statement):
    statement.text = statement.text.translate(str.maketrans('', '', string.punctuation))
    return statement

# remove ą, ę, etc.
def remove_accents(statement):
    statement.text = unidecode.unidecode(statement.text)
    return statement
