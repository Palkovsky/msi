import requests, re, csv, bs4
from itertools import chain

def parse(songname):
    url = "https://www.tekstowo.pl/piosenka,%s.html" % songname
    res = requests.get(url)
    print(songname, flush=True)
    assert res.status_code == 200
    html = bs4.BeautifulSoup(res.content)
    text_container = html.body.find("div", attrs={"class": "song-text"})
    assert text_container
    reps = [
        (r"Tekst piosenki:|Poznaj historię zmian tego tekstu|\"|\'|\"", r""),
        (r"Ref.?:", r""),
        (r"\d\.\s*", r""),
        (r"\(?x\s*\d\)?", r""),
        (r"\(?\d\s*x\)?", r""),
        (r"(\r\n)+", r"\n"),
        (r"\n+", r"\n")
    ]
    text = text_container.text
    for (old, new) in reps:
        text = re.sub(old, new, text)
    return text.strip()

def windows(iterable, n):
    i = 0
    l = list(iterable)
    sz = len(l)
    while i < sz:
        yield l[i:i+n]
        i += n

def chunks(label, text, window_size=1):
    lines = text.split('\n')
    for sample in windows(lines, window_size):
        joined = ' '.join(sample).strip()
        if len(joined) > 10:
            yield (label, joined)

def gensongs(author, *songs):
    author = author.lower().replace(" ", "_")
    songs = (song.lower().replace(" ", "_") for song in songs)
    return ("%s,%s" % (author, song) for song in songs)

if __name__ == "__main__":
    # Disco polo songs
    label1 = "disco"
    songs1 = chain(
        gensongs("radio chlew",
                 "alie olie", "wodko moja", "bydlak", "bo w naszej wsi", "lody dla rolnikow", "za te pachy spocone", "zabawa na 100", "wioska_moja_kochana__feat__zizej_skwarka___ja_jem_skwarki_"),
        gensongs("ja_jem_skwarki",
                 "rolnicze disko"),
        gensongs("akcent_pl_",
                 "przez_twe_oczy_zielone", "jesli_kochasz_1", "szczesliwa_gwiazda", "moja_gwiazda", "dziewczyna_z_klubu_disco_1", "dlaczego_1", "dziewczyna_ze_snu", "kochana_wierze_w_milosc", "krolowa_nocy_1"),
        gensongs("boys",
                 "bialy mis", "szalona", "biba", "ale numer", "3_x_hej", "dlaczego"),
        gensongs("classic",
                 "bez ciebie", "bilet do nieba", "chod__ze_mna", "dlaczego ty", "oczy czarne", "hej czy ty wiesz", "do widzenia", "dajmy sobie szanse"),
        gensongs("cliver",
                 "agnieszka", "ale wiem", "blondyneczka", "chod__kochanie", "co za noc", "cyganeczka zosia", "daj mi buzi mala", "dziewczyna bonda"),
        gensongs("toples",
                 "ale jazda", "polski slub", "pasja", "kobiety_mezczyzni", "cialo_do_ciala", "dominika", "bede twoj", "ewa", "na_sercu_rany", "tesknie", "sasiadka"),
        gensongs("skaner",
                 "afryka", "amore", "blondi", "ilona", "kasyno", "nadzieja", "lato_w_kolobrzegu", "wiosna", "julianna", "wakacyjny_romans"),
        gensongs("shazza",
                 "bierz_co_chcesz_", "egipskie noce", "historia_pewnej_milosci", "jestem_zakochana", "male_pieski_dwa", "tak_bardzo_zakochani"),
        gensongs("fanatic",
                 "czarownica", "ania", "daj_mi_kase_1", "dziewczyna nieznajoma", "hello", "ewa", "impreza", "jedna noc", "jak_sie_masz_kochanie", "szare dni", "wakacyjna_milosc", "wielki bal", "zabierz mnie"
        )
    )

    # Hip-hop songs
    label2 = "hiphop"
    songs2 = chain(
        gensongs("paktofonika",
                 "19", "2kilo", "a_robi_sie_to_tak", "az_strach_pomyslec", "gdyby", "ja_to_ja"),
        gensongs("peja",
                 "997", "bragga", "co_cie_boli", "glucha_noc", "ja_pierdole__feat__rdw__brahu_"),
        gensongs("rogal_ddl",
                 "ebe_ebe_1", "666", "156", "psycho", "kalashnikov", "abrakadabra_1", "prorapera_1", "turborower_1"),
        gensongs("waszka_g",
                 "zycie_ostre_jak_maczeta", "biedacy", "zeby_cos_zarobic_feat_bonus_rpk"),
        gensongs("bonus_bgc",
                 "angel_of_luj", "na_lazarskim", "gruby_rozpierdol__diss_tede_", "moje_miasto", "dobrze_fhuj_ft_bodychrist", "lazarnica"
        )
    )

    with open("dataset.csv", 'w', encoding="utf-8") as fil:
        writer = csv.writer(fil)
        for (label, songs) in ((label1, songs1), (label2, songs2)):
            samples = (sample for song in songs for sample in chunks(label, parse(song)))
            writer.writerows(samples)

