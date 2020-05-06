from random import choice
from chatterbot.logic import LogicAdapter
from chatterbot.comparisons import LevenshteinDistance
from chatterbot.conversation import Statement

class JokeAdapter(LogicAdapter):
    def __init__(self, chatbot, **kwargs):
        super().__init__(chatbot, **kwargs)

    def can_process(self, statement):
        phrases = [
            "opowiedz kawal",
            "opowiedz dowcip",
            "opowiedz mi dowcip",
            "opowiedz zart",
            "opowiedz mi aart",
            "rozbaw mnie",
            "opowiedz coś śmiesznego",
            "znasz jakis kawal",
            "znasz jakis dowcip"
        ]
        best = max([LevenshteinDistance().compare(statement, Statement(p.lower())) for p in phrases])
        return best > 0.8

    def process(self, input_statement, additional_response_selection_parameters):
        jokes = [
            '''
            Studniówka Tuska.

            Rudy cudotwórca przemawia, chwaląc się osiągnięciami swego rządu. Mija godzina, dwie, trzy... Premier śmiało przechodzi od osiągnięcia do osiągnięcia.
            - A tuż pod Mszczonowem Platforma Obywatelska doprowadziła do uruchomienia nowej, ekologicznej elektrowni, gdzie, zamiast węgla, pali się torfem - chwali się Tusk.
            Głos z sali:
            - Ale ja tam byłem, tam nie ma żadnej elektrowni!
            Premier niezrażony peroruje dalej:
            - A dzięki staraniom Platformy Obywatelskiej, niedaleko Jasła, wybudowaliśmy eksperymentalny odcinek autostrady siedmiopasmowej!AdapterMethodNotImplementedError()
            Tenże głos z sali:
            - Ale tam k...a nie ma żadnej autostrady!
            Nie wytrzymał w końcu Schetyna i wkur...y krzyczy:
- A ty, gościu, zamiast wozić się po Polsce, lepiej byś TVN24 pooglądał!
            ''',
            '''
            Żarcik  o Tusku w stylu angielskim

            Dziadek pokazuje małemu Donaldowi album z lat młodości.
            - Popatrz, Doniu, tutaj byłem w wojsku.
            Donald pyta - a kim jest ten pan z wąsami?
            - To był bardzo, ale to bardzo zły człowiek, wnusiu.
            - A dlaczego dziadziu trzymasz wyprostowaną prawą rękę w jego kierunku?
            Na to dziadek - Krzyczę do niego: "Hejże hola! Zatrzymaj się bardzo zły człowieku!"
            ''',
            '''
            Dowcip rybak i złota rybka

            Siedzi rybak nad morzem i łowi ryby. Nagle patrzy a tu na wędce trzepoce się mu złota rybka. Wyciągnął ją czym prędzej i się pyta:
            - Czy ty rybko jesteś ze złota?
            - Nie - odpowiedziała rybka - ja jestem z Platformy. - Zafrasował się stary rybak...
            - No to ty nie spełniasz życzeń? - zapytał ją.
            - Nie. - odpowiedziała rybka - ja tylko obiecuję.
            ''',
            '''
            Kawał o Castro, Putinie i Tusku

            Siedzą, każdy na swojej chmurce, Castro, Putin i Tusk, przyglądają się na ziemię i płaczą. Pan Bóg przechadzał się i zbliżywszy się do Castro, pyta:
            - Co tak płaczesz synu?
            - Panie Boże - odpowiada Castro zalany łzami - całe życie gnębiłem ten biedny naród, głodziłem, więziłem, co ja im zostawiłem, tylko głód i ubóstwo.
            - Nie martw się - odpowiada Pan Bóg - dzięki twojemu reżimowi wychowałeś ich na zahartowanych ludzi, poradzą sobie.
            Pan Bóg podchodzi do Putina i pyta, czemu tak płacze.
            - Panie Boże - mówi Putin - tak gnębiłem tych moich Rosjan i goniłem do pracy, zamykałem do więzienia, co ja im zostawiłem?
            - Nie martw się - odpowiada Pan Bóg - Rosjanie to dzielny naród, poradzą sobie wyjdą na ludzi.
            Wreszcie podchodzi Pan Bóg do płaczącego Tuska, patrzy ponad jego ramieniem na ziemię, siada koło niego i zaczyna płakać razem z nim.
            ''',
            '''
            TVN i krytyka opozycji

            Szef TVN zarządził dziś rano odprawę i mówi do dziennikarzy: "Już po wyborach. Przez dwa lata waliliśmy w rząd. Nie może tak dalej być. Jesteśmy elegancką, kulturalną, inteligentną i przyzwoitą telewizją na poziomie. Od dziś krytykujemy opozycję!"
''',
            '''
            Dowcip jak Tusk zwołuje radę ministrów

            Premier Donald Tusk zwołuje radę ministrów do swojego gabinetu i mówi do zgromadzonych ministrów.
            - Mam tutaj 4 przyciski czerwony, żółty, niebieski i zielony:
            - jak wcisnę przycisk czerwony to na drogach nie będzie wypadków i będzie 3000 km autostrad
            - jak wcisnę przycisk żółty to 3 miliony emigrantów wróci do Polski i przyjadą lekarze z zachodu żeby pracować w naszych szpitalach
            - jak wcisnę przycisk niebieski to nie będzie podatków a wynagrodzenia wzrosną o 200%
            - jak wcisnę przycisk zielony to nie będzie przymrozków i susz i jabłka i inna żywność już nigdy nie będzie drożeć.
            W tym momencie przerywa mu minister rolnictwa z PSL i mówi - a u nas w Ciechanowie to była babcia Józia, co miała 4 nocniki w przedpokoju - czerwony, żółty, niebieski i zielony.
            - No i co? No i co? - pyta Donald
            - I zesrała się na schodach...
            ''',
            '''
            Żarcik jak Donald Tusk likwidował urzędy

            Premier Tusk zlikwidował wszystkie ministerstwa, a w ich miejsce powołał 3 nowe:
            1. Zdrowia
            2. Szczęścia
            3. Pomyślności
            ''',
            '''
            Dowcip o Tusku aniołku

            Tusk umarł i dostał możliwość wyboru miedzy Niebem a Piekłem. W Niebie widzi modlących się, grających na harfach, śpiewających. Nuda. Za to w Piekle balanga, popijawy, panienki, wesołe towarzystwo... Wybrał Piekło. Ale jak tam trafił, balangi już nie było, tylko tradycyjnie umieścili go w kotle ze smołą. Tusk się poskarżył się diabłu, że nie tak miało być na co ten odparł: "Sam to wiesz najlepiej. Kampania wyborcza ma swoje prawa."
            ''',
            '''
            Oda do PO

            Grzmią Polsaty, TVN-y, wielkiej PO głosząc chwałę.
            Nigdy PIS-u już nie będzie, Donald, Donald, ueber alles.
            Niesiołowski w geście triumfu już winduje ich sondaże,
            Jeszcze nawet nie zaczęli, a już chwalą swe miraże.
            Miller męskość swoją chwalił, a jak skończył sami wiecie,
            Oni jeszcze nie zaczęli, a już mówią o repecie.
            Więc POkory ucz się bracie i rachunek zrób sumienia,
            Byś z sondażem tak wysokim, nie spadł z Olimpu sklepienia.
            ''',
            '''
            Satyra dla Tuska

            Ryże włosy, puste słowa,
            Spuchła twarz bazyliszkowa
            Wytrzesz oczu nieprzeciętny
            By faflunić - zawsze chętny.
            Cud miał jak w Irlandii zrobić
            Lecz wziął puszkę, by zarobić
            Bo gdy w głowie samo siano
            Gdzie nie poszedł, go wyśmiano!

            Ciągle chamski, wredny, spięty,
            Ciągle szczeka bez zachęty
            Kraj obraża, prezydenta,
            Świat takiego nie pamięta!

            Nie ma planu do rządzenia,
            Lecz ma chętkę do siedzenia,
            Rządom swoim nie chce sprostać,
            Prezydentem chce już zostać.
            Służba zdrowia czy górnicy,
            Każdy żal ma na ulicy,
            Wiosną każdy będzie skory
            Zagłosować na Kaczory.

            I prezydent mu pomaga,
            Widząc, że Tusk to łamaga,
            niedołężny, snobistyczny,
            wręcz IMPOTENT POLITYCZNY.

            Donald pomoc tę odrzuca,
            Błotem Polskę chce obrzucać,
            Nie naprawiać, co tu gadać,
            On chce niszczyć i rozkradać
            ''',
            '''
            Urzędnik z PO we Francji

            Platformerski urzędnik udał się w oficjalną podróż do Francji. Jednym z punktów wizyty była kolacja u francuskiego odpowiednika. Widząc jego wspaniałą willę, z obrazami wielkich mistrzów na ścianach, pyta się, jak on zapewnia sobie taki poziom życia ze skromnej pensji urzędnika.
            Francuz zaprasza go do okna:
            - Widzi pan tę autostradę?
            - Tak.
            - Ona kosztowała 20 miliardów euro, firma wypisała fakturę na 25, a różnicę przekazała mi. Dwa lata później urzędnik francuski udaje się do Polski i odwiedza swojego polskiego odpowiednika. Kiedy podjeżdża pod domostwo, widzi wspaniały pałac więc pyta:
            - Dwa lata temu stwierdził pan, że prowadzę książęce życie, ale w porównaniu do pana... Platformerski urzędnik podchodzi do okna:
            - Widzi pan tam autostradę?
            - Nie.
            - No właśnie.
            ''',
            '''
            Dowcip o CUDACH

            Pewien fircyk - bez krępacji, CUD obiecał całej nacji.
            Trafnie uznał - lud to kupi! Lud naiwny, ciemny, głupi.
            No i młodzież uwierzyła, do urn wartko podążyła.
            I we Wronkach i w Półtusku, dali głos Donaldu Tusku.
            No nareszcie jest nadzieja! Naród wybrał czarodzieja.
            Media wielce mu pomogły, nawet Doda wzniosła modły.
            To co najpierw obiecywał, po wyborach odwoływał.
            Toż to kawał hipokryty, rude włosy, wzrok kosmity.
            Serki, jabłka podrożały, a indeksy pospadały!
            No i w końcu stał się CUD, Czary mary... mamy SMRÓD !!!
            ''',
            '''
            PO jak PEŁO

            PO - POstępowy POpapraniec
            PO - Puste Obietnice

            Zasady walki politycznej w PO:
            1. Jeśli warto o coś walczyć to tym bardziej warto walczyć nieuczciwie.
            2. Nie kłam, nie oszukuj, nie kradnij ... bez potrzeby.
            3 .Szczera odpowiedź może przysporzyć tylko problemów.
            4. Fakty, aczkolwiek interesujące, są bez znaczenia.
            5. Prawda jest zmienna.
            6. Obietnica nie jest gwarancją.
            ''',
            '''
            Wspomnienie z MISIA!!!

            Dzień dobry, cześć i czołem! Pytacie skąd się wziąłem?
            Jestem wesoły Donek, marszałkiem zaś jest Bronek.
            Zdrożała żywność, prąd i gaz,
            mogę obiecać coś jeszcze raz...
            ''',
            '''
            Tusk i Klich odwiedzili jedną z warszawskich podstawówek.

            Podczas dyskusji z uczniami Tusk zapytał:
            - Co to jest tragedia? Czy ktoś mógłby podać przykład?
            Dziewczynka z pierwszej ławki podniosła rękę:
            - Gdyby mój przyjaciel, który mieszka na wsi, bawił się na polu i został rozjechany przez traktor - to byłaby tragedia.
            - Nie. - odpowiada Klich - to byłby wypadek
            Zgłasza się kolejne dziecko:
            - Gdyby autobus, który odwozi 50 dzieci do szkoły, miał wypadek, w którym zginęliby wszyscy pasażerowie - to byłaby tragedia.
            - Też nie - odpowiada tym razem Tusk - to byłaby wielka strata.
            Czy ktoś ma inne pomysły?
            W klasie cisza. Nikt nie chce się zgłosić.
            Nagle odzywa się Jasiu:
            - Gdyby samolot, w którym lecieli pan premier i pan minister został trafiony przez pocisk i rozpadł się na kawałki - to byłaby tragedia.
            - Brawo! - woła Tusk - Możesz nam powiedzieć dlaczego uważasz, ze to była by tragedia?
            Na to Jasiu:
            - Dlatego, że to na pewno nie byłaby wielka strata i prawie na pewno nie byłby to wypadek...
            ''',
            '''
            Pierwszy tydzień rządów Donalda Tuska

            Poniedziałek Donald Tusk wyczarterował 10.000 jumbo-jetów. 3 miliony Polaków wraca do kraju.
            Wtorek Zarządzeniem premiera Donalda Tuska zlikwidowano wypadki samochodowe.
            Środa
            Donald Tusk obniżył podatki --- od środy PIT, CIT i VAT w Polsce wynoszą 1%, jednocześnie Donald Tusk zwiększył wynagrodzenia --- pielęgniarka zarabia średnio 5890 zł netto, a lekarz 25630 zł netto. Nauczyciele zarabiają od środy 8300 zł netto, a urzędnicy 6250. Zachodnie granice Polski przekraczają tysiące lekarzy z Niemiec Francji i Irlandii by podjąć pracę w polskich szpitalach.
            Czwartek
            W czwartek Donald Tusk gwałtownie podniósł prestiż Polski na świecie --- UE ogłosiła wprowadzenie Nicei i pierwiastka jednocześnie, Niemcy zrezygnowały z Gazociągu Północnego, a bałtycką rurą będzie transportowane mięso z Polski do Rosji.
            Piątek
            W piątek Donald Tusk wybudował 2500 kilometrów autostrad 840 pływalni i 320 stadionów. Hanna Gronkiewicz-Waltz --- nowa minister infrastruktury - przecięła 6000 wstęg w ciągu 18 godzin
            Sobota
            Donald Tusk rozdał akcje, średnio 70 tysięcy na głowę. Ponadto wprowadził poszóstne becikowe i
            ulgę na dziecko --- bez kozery - "pińcet" tysięcy.
            Niedziela
            Po zrobieniu tych wszystkich cudów, siódmego dnia, Donald Tusk odpoczął.
            P.S.
            Wieczorem w niedzielę od niechcenia Donald Tusk zniósł przymrozki, by jabłka już nigdy nie drożały.
''',
            '''
            Donio u dzieci

            Donald Tusk wybrał się z prezentami dla dzieci na ubogie tereny wschodniej Polski.
            - Grzeczne były dzieci? - pyta Tusk.
            - Bardzo!
            - Świetnie.. ale dlaczego są takie chude ?
            - Bo nie jedzą.
            - Nie jedzą... a jak nie jedzą, to prezentów nie dostaną!
            ''',
            '''
            Donald do Schetynki

            Donald Tusk do Schetyny:
            - Kiedy powiedziałem, że Polska jest wyspą ekonomicznej stabilizacji,
            to potem spałem już jak dziecko!
            - To znaczy?
            - Płakałem całą noc i dwa razy zabrudziłem łóżko!
            ''',
            '''
            Urzędnik PO uzdrowiciel

            Kolega Tuska z Platformy Obywatelskiej do wnuczka:
            - Obniżymy podatki o połowę, wybudujemy bezpłatne autostrady,
            uzdrowimy służbę zdrowia, nakarmimy wszystkie dzieci, uczniom
            kupimy laptopy ... - A jutro jaką bajkę ci opowiedzieć ?
            ''',
            '''
            Dzień Wykształciucha

            8:00 - W-F -"Poranne Ćwiczenia skłonów, klekania i kłaniania
            się w kierunku Brukseli i Moskwy"
            9:00 - Laboratorium - "Szybkie mycie garów-nadciąga Irlandia"
            10:00 - Wykład - "Mocheroznawstwo-Prowadzi S.Niesiołowski"
            11:00 - Wykład - "Jak posiąść Profesure nie będąc Profesorem"
            prowadzi p.o. profesora - Bartoszewski
            12:00 - Przerwa obiadowa ( w menu: tanie jabłka i kurczaki)
            12:30 - Ćwiczenia - "Szybka łapa - nie daj sie złapać"
            Prowadzi POseł Beata Sawicka
            14:00 - Ćwiczenia - POprawna mowa - dziś literka "r" -
            prowadzi D.Tusk i H.G. Waltz
            15:00 - Wykład - "Swoje chwalicie cudzego nie znacie -
            Jamajka i Tel Awiw" - Prowadzi B. Komorowski i
            B.Geremek
            16:00 - Seminarium - "Idealna prasa w Polsce" Prowadzi
            Profesor A. Michnik
            17:00 - Laboratorium - "Techniki nagrywania" - prowadzi duet
            Sekielski i Morozowski
            18:00 - Wykład - "Kulturalne zachowanie" - prowadzi T. Lis
            ''',
            '''
            Co łączy Szwejka i Bogdana Klicha?

            Jaka jest różnica między dzielnym wojakiem Szwejkiem, a Minister Obrony Narodowej w rządzie Tuska - Bogdanem Klichem?
            - Dzielny wojak Szwejk był cwaniakiem, a udawał głupca, Klich natomiast nie służył w armii austro-węgierskiej...
            ''',
            '''
            Bajki o peło i Donku

            Czym różnią się normalne bajki od bajek podawanych przez polityków Platformy Obywatelskiej?
            W normalnych bajkach wszystko się dzieje za 7 rzekami, za 7 morzami, za 7 górami...
            W bajkach podawanych przez polityków PO - jak podaje TVN24, jak podaje Gazeta Wyborcza.
            '''
        ]
        result = Statement(choice(jokes))
        result.confidence = 1.0
        return result
