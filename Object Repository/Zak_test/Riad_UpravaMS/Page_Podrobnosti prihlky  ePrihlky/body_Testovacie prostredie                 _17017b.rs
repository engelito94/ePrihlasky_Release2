<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Testovacie prostredie                 _17017b</name>
   <tag></tag>
   <elementGuidId>699ab512-45f9-4b9f-bf69-fb992295a582</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.govuk-frontend-supported</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>33cc08d2-f9fc-47b7-8396-539dc955095e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-frontend-supported</value>
      <webElementGuid>0f97389d-0565-4149-8386-4eddd21d0eca</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    

        
            Testovacie prostredie
        
        
            
                error
                Dôležité: Tento portál je momentálne v pilotnej verzii a je dostupný len pre vybrané školy. Viac informácií.
            
        
    
        
            
                
                    
                        

                            
                                Oficiálna stránka

                                
                                    e-Gov
                                    verejnej správy SR
                                    arrow_drop_down
                                
                            
                            
                                
                                    
                                        Doména gov.sk je oficiálna
                                        Toto je oficiálna webová stránka orgánu verejnej moci Slovenskej republiky. Oficiálne stránky využívajú najmä doménu gov.sk. Odkazy na jednotlivé webové sídla orgánov verejnej moci nájdete na tomto odkaze.
                                    
                                    
                                        Táto stránka je zabezpečená
                                        Buďte pozorní a vždy sa uistite, že zdieľate informácie iba cez zabezpečenú webovú stránku verejnej správy SR. Zabezpečená stránka vždy začína https:// pred názvom domény webového sídla.
                                    
                                
                            
                        
                        
                            
                                slovenčina
                                arrow_drop_down
                            
                            
                                        
                                            
                                                slovenčina
                                            
                                        
                            
                        
                    
                
            

            
                
                    
                        landscape
                    
                    
                        Elektronické prihlášky do škôl
                    
                
                
                    
                        
                            Menu
                            
                                menuicon
                            
                        
                    
                            
                                
                                    
                                        KL
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const controlSettingsPodrobnostiPrihlasky = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        validovaneSNesuladom2: &quot;Validované s nesúladom:&quot;,
        validovaneSNesuladom3: &quot;Nevalidované voči RFO&quot;,
        validovaneSNesuladom4: &quot;Prebieha overenie údajov&quot;,
        zaslaneDoSkol: &quot;Zaslané do škôl:&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaStavPrihlasky: &quot;Stav prihlášky&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
        '8042': 'Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.',
        '8086': 'Zadajte dátum v aktuálnom roku menší alebo rovný aktuálnemu',
        '8087': 'Informáciu o nástupe dieťaťa ste úspešne zaevidovali.',
        datumPodaniaLabel: &quot;Dátum podania&quot;,
        udajSaNezhodujeRFOPopup: &quot;Údaj sa nezhoduje s údajom rodiča v RFO.&quot;,
        udajSaNezhodujeRFODietaPopup: &quot;Údaj sa nezhoduje s údajom dieťaťa v RFO.&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        zaevidujteNastupDescription: &quot;Pre zaevidovanie informácie, či dieťa nastúpi alebo nenastúpi do školy, je potrebné vybrať jednu z možností nižšie. V prípade výberu možnosti Nastúpi, bude pre všetky ostatné školy, kde dieťa prijali, automaticky označené, že nenastúpi. Nástup je možné zaevidovať alebo zmeniť do $datum$.&quot;,
        podlaRIS: &quot;podľa RIS&quot;,
        nieJeKDispoziciiZRIS: &quot;(Tento údaj nie je k dispozícii z RIS.)&quot;,

        vyberAlternativuRadio: {
            label: 'Vyberte alternatívu:',
            options: [
                {
                    label: &quot;Nastúpi&quot;,
                    value: &quot;1&quot;,
                },
                {
                    label: &quot;Nenastúpi&quot;,
                    value: &quot;0&quot;,
                }
            ],
            direction: 'column',
            required: true,
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;
                }
            ],

        },


        // pdf
        pdfTitleZS: &quot;Prihláška na vzdelávanie v základnej škole - náhľad&quot;,
        pdfTitleMS: &quot;Žiadosť o prijatie dieťaťa na predprimárne vzdelávanie - náhľad&quot;,
        pdfDescription: &quot;Pilotná verzia portálu Elektronických prihlášok do škôl. Náhľad nemožno použiť ako náhradu oficiálnej prihlášky.&quot;,

        pdfVseobecneInformacieHeader: &quot;Všeobecné informácie&quot;,
        pdfVseobecneValidaciaRfo: &quot;Výsledok validácie s RFO(placeholder):&quot;,
        pdfVseobecneIdentifikatorPrihlasky: &quot;Identifikátor prihlášky:&quot;,
        pdfVseobecneDatumPodania: &quot;Dátum podania:&quot;,
        pdfVseobecneSkolskyRok: &quot;Školský rok:&quot;,
        pdfVseobecneSposobPodania: &quot;Spôsob podania:&quot;,
        pdfVseobecneStavPrihlasky: &quot;Stav prihlášky:&quot;,
        pdfVseobecneSpadovaPrihlaska: &quot;Spádová prihláška&quot;,
        pdfVseobecneDuplicitnaPrihlaska: &quot;Duplicitná prihláška&quot;,
        pdfVseobecneSurodenecNaSkole: &quot;Súrodenec na škole:&quot;,
        pdfVseobecnePoznamkaSkoly: &quot;Poznámka školy:&quot;,

        pdfDietaHeader: &quot;Podrobnosti o dieťati&quot;,
        pdfDietaMeno: &quot;Meno a priezvisko dieťaťa:&quot;,
        pdfDietaRodnePriezvisko: &quot;Rodné priezvisko:&quot;,
        pdfDietaRodneCislo: &quot;Rodné číslo:&quot;,
        pdfDietaDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfDietaPohlavie: &quot;Pohlavie:&quot;,
        pdfDietaMiestoNarodenia: &quot;Miesto narodenia:&quot;,
        pdfDietaNarodnost: &quot;Národnosť:&quot;,
        pdfDietaStatnaPrislusnost: &quot;Štátna príslušnosť:&quot;,
        pdfDietaMaterinskyJazyk: &quot;Materinský jazyk:&quot;,
        pdfDietaInyMaterinskyJazyk: &quot;Iný materinský jazyk:&quot;,
        pdfDietaAdresaTP: &quot;Adresa trvalého pobytu:&quot;,
        pdfDietaAdresaZP: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava (ak sa nezdržiava na adrese trvalého pobytu):&quot;,

        pdfSkolaHeader: &quot;Zoznam škôl&quot;,
        pdfSkolaNazov: &quot;Názov školy:&quot;,
        pdfSkolaAdresa: &quot;Adresa:&quot;,
        pdfSkolaVyucovaciJazyk: &quot;Vyučovací jazyk:&quot;,
        pdfSkolaStavPrihlasky: &quot;Stav prihlášky:&quot;,

        pdfZZHeader: &quot;Podrobnosti o zákonných zástupcoch dieťaťa&quot;,
        pdfZZMeno: &quot;Meno a priezvisko:&quot;,
        pdfZZDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfZZCisloElektronickejSchranky: &quot;Číslo elektronickej schránky:&quot;,
        pdfZZAdresaBydliska: &quot;Adresa bydliska:&quot;,
        pdfZZEmail: &quot;Email:&quot;,
        pdfZZTelefon: &quot;Telefónne číslo:&quot;,
        pdfZZ2Suhlas: &quot;Súhlas druhého zákonného zástupcu:&quot;,
        pdfZZ2DovodNezabezpeceniaSuhlasu: &quot;Dôvod nezabezpečenia súhlasu:&quot;,

        pdfPrilohyHeader: &quot;Prílohy:&quot;,

        pdfDoplnujuceInformacieHeader: &quot;Doplňujúce informácie o dieťati:&quot;,
        pdfDPDPozadovanaVychova: &quot;Požadovaná výchova:&quot;,
        pdfDPDStravovanie: &quot;Záujem o stravovanie v školskej jedálni:&quot;,
        pdfDPDDruzina: &quot;Záujem o Školský klub detí:&quot;,
        pdfDPDSVVPotreby: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním:&quot;,
        pdfDPDPopisSP: &quot;Popis znevýhodnenia / nadania:&quot;,
        pdfDPDPoznamka: &quot;Poznámka:&quot;,
        pdfDPDZiadam: &quot;Žiadam o prijatie dieťaťa na:&quot;,
        pdfDPDCelodenna: &quot;celodennú výchovu a vzdelávanie&quot;,
        pdfDPDPoldenna: &quot;poldennú výchovu a vzdelávanie&quot;,

        pdfFooter1: &quot;Generované ${dateFormatted} o ${timeFormatted} pilotnou verziou portálu Elektronických prihlášok do škôl.&quot;,
        pdfFooter2: &quot;© 2025, eprihlasky.iedu.sk&quot;,

    };




    
        
            chevron_left
        
        
            Späť
        
    
    

    
        
            
                Podrobnosti prihlášky
                
                    
                        Pridávajte dokumenty a upravujte informácie podľa potreby.
                    
                
            
            
                
                    Exportovať v PDF
                
                
                    add
                    Pridať prílohu
                
                
                    Vyžiadať prílohu
                
            
        

        
            
                info
                
                    Zaevidujte nástup dieťaťa manuálne.
                    
                        
                            Dieťa nastúpi do školy.
                            Dieťa nenastúpi do školy.
                        
                    
                
                
                    Vybrať stav
                
            

            
                check_circle

    
    
        
            
            
        
        
            
            
        
    


            
        

        
        

        
            Všeobecné informácie
            
                
                    
                        numbers
                        P-2025-90345
                    
                
                
                    
                        child_care
                        Tibor Zelený
                    
                
                
                    
                        calendar_month
                        
                            29.05.2025
                            Upraviť
                        
                        
                            
                                

    
        Dátum podania
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                            
                            
                                Späť
                                Uložiť
                            
                        
                    
                
                
                    
                        bookmark
                        2025/2026
                    
                
                
                    
                        house
                        Zaslané do škôl: 1
                    
                
                
                    
                        mail
                        Papierovo
                    
                
                
                    
                        stars
                        
                    
                        Duplicitná prihláška
                        clear
                    
                        
                            Vyberte
                            keyboard_arrow_down_rounded

                            
                                Vyberte
                            Duplicitná prihláškaSpádová prihláškaSúrodenec na škole
                        
                    
                
                
                    
                        description
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                
                
                    Nástup dieťaťa zaevidoval
                    
                
                
                    Poznámka školy
                    
                        -
                        Upraviť
                    
                    
                        
                            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/500
        
    
    


                        
                        
                            Späť
                            Uložiť
                        
                    
                
            
        

        
            
                
                    Podrobnosti o dieťati
                
                Upraviť
            
            
                
                    Meno a priezvisko dieťaťa
                    Tibor Zelený
                
                
                    Rodné priezvisko
                    Zelený
                
                
                    Rodné číslo
                    201206/0149
                
                
                    Dátum narodenia
                    06.12.2020
                
                
                    Pohlavie
                    muž
                
                
                    Miesto narodenia
                    Lučenec
                
                
                    Národnosť
                    slovenská
                
                
                    Štátna príslušnosť
                    Slovenská republika
                
                
                    Materinský jazyk
                    slovenský
                
                
                    Iný materinský jazyk
                    -
                
                
                    Adresa trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Povinné predprimárne vzdelávanie aktuálne v
                    -
                
            
        

        
            Zoznam škôl
            
                    
                        1
                        Informácie o škole č.1
                    
                    
                        Stav prihlášky
                        
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                    
                    
                        Názov školy
                        Materská škola
                    
                    
                        Adresa
                        05912, Hôrka 142
                    
                    
                        Vo vyučovacom jazyku
                        slovenský
                    
                
        

        
            
                Podrobnosti o zákonných zástupcoch dieťaťa
                Upraviť
            
            
                Informácie o zákonnom zástupcovi č.1:
                
                    
                        Meno a priezvisko
                        Emanuel Zelený
                    
                    
                        Dátum narodenia
                        22.09.1980
                    
                    
                        Číslo elektronickej schránky
                        E2125898753
                    
                    
                        Adresa bydliska
                        Gemerská cesta 18, Lučenec, Slovenská republika
                    
                    
                        Email
                        mail@mail.sk
                    
                    
                        Telefónne číslo
                        +421000222111
                    
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                
                
                Informácie o zákonnom zástupcovi č.2:
                
                    
                        Meno a priezvisko
                        Elena Hronová
                    
                    
                        Dátum narodenia
                        15.04.1985
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa bydliska
                        Námestie Svätého Egídia 45, Poprad, Slovenská republika
                    
                    
                        Email
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas druhého zákonného zástupcu
                        nie
                    
                    
                        Dôvod nezabezpečenia súhlasu
                        Zdravotný - nevie sa podpísať
                    
                

                Druhý zákonný zástupca nie je známy.
            
        

        
            
                Prílohy a doplňujúce informácie o dieťati
                Upraviť
            
            
                Prílohy

                
                    
                        article
                        Dokument.pdf / Čestné vyhlásenie zákonného zástupcu
                    
                

                

                Doplňujúce informácie o dieťati
                
                    
                        Celodenná výchova
                        áno
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        áno
                    
                    
                        Popis znevýhodnenia / nadania
                        testovací popis
                    
                    
                        Poznámka
                        -
                    
                
            
        

        
        
        
            bla bla
        
        
            Údaj sa nezhoduje s údajom rodiča v RFO.
        
        
            Údaj sa nezhoduje s údajom dieťaťa v RFO.
        

    



    

    
        assignment_turned_in
        Zaevidujte nástup dieťaťa
        

        
            

    
    
        Vyberte alternatívu:
        (nepovinné)
    
    
    
    
                        
                        Nastúpi 
                         
                        
                        Nenastúpi 
                         
    

        

        
            Späť
            Dokončiť
        
    



    
    
        
            
                
                    Items
                    
                        
                            
                                Vyhlásenie o prístupnosti
                            
                        
                        
                            
                                Kontakt na prevádzkovateľa
                            
                        
                        
                        
                            
                                Mapa stránky
                            
                        
                        
                            
                                Metodické usmernenia
                            
                        
                        
                            
                                Oznamy
                            
                        
                        
                    
                    
                        Prevádzkovateľom služby je Ministerstvo školstva, výskumu, vývoja a mládeže Slovenskej republiky.
                        
                        © 2013 - 2025, eprihlasky.iedu.sk
                    
                
                
                    
                        
                    
                
            
        
    


        
    
    
    







    



     const isLogged = true;
    const maTD = true;
    if (isLogged === true || maTD === true) {
        window.userData = {&quot;nameId&quot;:&quot;ee961ca2-3f59-4e23-b436-00a6e1075678&quot;,&quot;tokenDescriptor&quot;:&quot;c6934a87-64ec-4232-adf8-56c18b811a22:9db37aca172615ab8ec4f0d8f69cf2b74702c56b19aa8b99a5ec826006b4f122&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930571647&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgSUQ9Il9mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI19mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT5yN055bFVTR1NGMlpkVnVHNHlEVTN2eTdJTDFHV1E3dVlmUk5mQzBwbEo4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5VRVNiZDlDcTMzV3hta1hYU3dlaGpkcnNaNjlUUDJ0YnNlQWRQVnI0WFc3WWorRDRmQlJOQnhxbXZ2QTlYRnhYYWJTZmlpeGd4SFZNNjJ1SHVxeXZDNkh5VUpPeGZ5UHVhTlVPWXVlR3lzWjl4ZWczR1dIaHljN1JWRW5qWmloQVo2VkQ0UC9nWHM4YWdBRUR5Qms4dXlPY1VrVVZ6alFDa2xTMENhemV3eHdGMHRqSFhuTWlURTFBeU1EZTBFQUFMcFRrcWgyZy9zS0pzb1F6d0tjbzlZb1MvKzFPdHhzWWpsSGtkbjcvUWk5WmVabUc1eENKaGhBaHNnV2ZoZ3gxdHV4enZvbk43L0c5ejhlQWpHNTVyU0tpU21KNkYwMTBBUVA4bWxQVVB0R3ZFS09UVEFNWCtzeDYvcDBDeHd4dHF5RUN1V2FWT29aY2pCU1pFajkxL0E9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfYWJmNGQyYTMtMzQ1Yy00ZTJmLWIwNTUtNjQyOTllZmZmNWM1IiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmVlOTYxY2EyLTNmNTktNGUyMy1iNDM2LTAwYTZlMTA3NTY3ODwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDExOjMyOjExLjc3MloiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMToxNzoxMS43NzJaIiBTZXNzaW9uSW5kZXg9ImM2OTM0YTg3LTY0ZWMtNDIzMi1hZGY4LTU2YzE4YjgxMWEyMjo5ZGIzN2FjYTE3MjYxNWFiOGVjNGYwZDhmNjljZjJiNzQ3MDJjNTZiMTlhYThiOTlhNWVjODI2MDA2YjRmMTIyIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkthcm9sw61uYTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvUHJpZXp2aXNrbyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkxpZXRhasO6Y2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MDkuMDMuMTk5MDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5ERkVFMUJCNi0wMzhGLTQ2MDMtQUYxQi04RkU2QjA5MTIwOTU8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MTY0Nzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcxNjQ3PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930571647&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;birthNumber&quot;:null,&quot;genderCode&quot;:&quot;2&quot;,&quot;dateOfBirth&quot;:&quot;1990-03-09&quot;,&quot;name&quot;:&quot;Karolína&quot;,&quot;surname&quot;:&quot;Lietajúca&quot;,&quot;subjectGuid&quot;:&quot;acc1dadc-299a-4977-94bd-0b5e15e77a6b&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;identifikatorVIAM&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910012370&quot;,&quot;vybrataSaszNazov&quot;:&quot;Materská škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910012370&quot;,&quot;nazov&quot;:&quot;Materská škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-10&quot;}]}};
    }
    window.env = &quot;ZakTest&quot;;


    function loadJsCSHTML() {
        dtc.globalTranslations = {
            cookies: {
                title: &quot;Potvrdenie súhlasu s cookies&quot;,
                description: &quot;S cieľom zabezpečiť správne fungovanie tejto webovej lokality, ukladáme v niektorých prípadoch na vašom zariadení malé dátové súbory, tzv. cookie.&lt;br>Tieto súbory nám pomáhajú poskytovať kvalitnejšie služby, čím vám umožňujeme pohodlnejšie využívanie stránok.&lt;br>Taktiež používame analytické súbory, akceptovaním súhlasíte s ich používaním.&quot;,
                button: &quot;Súhlasím&quot;,
                buttonSecondary: &quot;Nesúhlasím&quot;
            },
            overlay: {
                closeButton: &quot;Zatvoriť&quot;,
                yes: &quot;Áno&quot;,
                no: &quot;Nie&quot;,
                ok: &quot;OK&quot;
            },
            errorSummaryBox: {
                ajaxError: {
                    title: &quot;Systémová chyba&quot;
                },
                title: &quot;Nasledujúce polia neboli vyplnené správne:&quot;,
                description: &quot;&quot;,
            },
            systemError: {
                systemError: 'Systémová chyba',
                server: 'Došlo k chybe pri komunikácii so serverom. V prípade pretrvávania chyby kontaktujte zákaznícku podporu.'
            },
            calendar: {
                days: [
                    'Nedeľa',
                    'Pondelok',
                    'Utorok',
                    'Streda',
                    'Štvrtok',
                    'Piatok',
                    'Sobota',
                ],
                months: [
                    'Január',
                    'Február',
                    'Marec',
                    'Apríl',
                    'Máj',
                    'Jún',
                    'Júl',
                    'August',
                    'September',
                    'Október',
                    'November',
                    'December',
                ]
            }
        };

        dtc.links = {
            home: &quot;/&quot;,
            pomocAPodpora: '/Pomoc-a-podpora',
            zakladneInformacie: '/Pomoc-a-podpora/Zakladne-informacie',
            akoPodatPrihlaskuNaZS: '/Pomoc-a-podpora/Ako-podat-prihlasku-na-ZS',
            akoPodatPrihlaskuNaMS: '/Pomoc-a-podpora/Ako-podat-prihlasku-na-MS',
            mojProfil: '/Moj-profil',
            dieta: '/Dieta',
            nastavenie: '/Nastavenie',
            prihlaska: '/Prihlaska',
            spatnaVazba: '/Spatna-vazba',
            prihlaskaOdoslana: '/Prihlaska-odoslana',
            oznamy: '/Oznamy',
            mapaStranok: '/Mapa-stranok',
            selectSaszCase: '/SelectSaszCase',
            ziadost: '/Ziadost',
            riaditel: '/Riaditel',
            rozhodnutia: '/Riaditel' + '?Rozhodnutia',
            selectSaszCase: '/SelectSaszCase',
            &quot;401error&quot;: '/Logout?returnUrl=/Error?code=401',
            vydatRozhodnutia: '/Riaditel/Vydat-rozhodnutia'
        };
    }

    

    

/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]</value>
      <webElementGuid>d9fd199a-7b6f-4997-94a7-3f543aad2429</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]</value>
      <webElementGuid>03e58374-b2a4-450b-b568-62189a30777d</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>75f632de-1800-47ca-9034-4bc52da912a5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
    

        
            Testovacie prostredie
        
        
            
                error
                Dôležité: Tento portál je momentálne v pilotnej verzii a je dostupný len pre vybrané školy. Viac informácií.
            
        
    
        
            
                
                    
                        

                            
                                Oficiálna stránka

                                
                                    e-Gov
                                    verejnej správy SR
                                    arrow_drop_down
                                
                            
                            
                                
                                    
                                        Doména gov.sk je oficiálna
                                        Toto je oficiálna webová stránka orgánu verejnej moci Slovenskej republiky. Oficiálne stránky využívajú najmä doménu gov.sk. Odkazy na jednotlivé webové sídla orgánov verejnej moci nájdete na tomto odkaze.
                                    
                                    
                                        Táto stránka je zabezpečená
                                        Buďte pozorní a vždy sa uistite, že zdieľate informácie iba cez zabezpečenú webovú stránku verejnej správy SR. Zabezpečená stránka vždy začína https:// pred názvom domény webového sídla.
                                    
                                
                            
                        
                        
                            
                                slovenčina
                                arrow_drop_down
                            
                            
                                        
                                            
                                                slovenčina
                                            
                                        
                            
                        
                    
                
            

            
                
                    
                        landscape
                    
                    
                        Elektronické prihlášky do škôl
                    
                
                
                    
                        
                            Menu
                            
                                menuicon
                            
                        
                    
                            
                                
                                    
                                        KL
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const controlSettingsPodrobnostiPrihlasky = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        validovaneSNesuladom2: &quot;Validované s nesúladom:&quot;,
        validovaneSNesuladom3: &quot;Nevalidované voči RFO&quot;,
        validovaneSNesuladom4: &quot;Prebieha overenie údajov&quot;,
        zaslaneDoSkol: &quot;Zaslané do škôl:&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaStavPrihlasky: &quot;Stav prihlášky&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8086&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zadajte dátum v aktuálnom roku menší alebo rovný aktuálnemu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8087&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Informáciu o nástupe dieťaťa ste úspešne zaevidovali.&quot; , &quot;'&quot; , &quot;,
        datumPodaniaLabel: &quot;Dátum podania&quot;,
        udajSaNezhodujeRFOPopup: &quot;Údaj sa nezhoduje s údajom rodiča v RFO.&quot;,
        udajSaNezhodujeRFODietaPopup: &quot;Údaj sa nezhoduje s údajom dieťaťa v RFO.&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        zaevidujteNastupDescription: &quot;Pre zaevidovanie informácie, či dieťa nastúpi alebo nenastúpi do školy, je potrebné vybrať jednu z možností nižšie. V prípade výberu možnosti Nastúpi, bude pre všetky ostatné školy, kde dieťa prijali, automaticky označené, že nenastúpi. Nástup je možné zaevidovať alebo zmeniť do $datum$.&quot;,
        podlaRIS: &quot;podľa RIS&quot;,
        nieJeKDispoziciiZRIS: &quot;(Tento údaj nie je k dispozícii z RIS.)&quot;,

        vyberAlternativuRadio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte alternatívu:&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;Nastúpi&quot;,
                    value: &quot;1&quot;,
                },
                {
                    label: &quot;Nenastúpi&quot;,
                    value: &quot;0&quot;,
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ],

        },


        // pdf
        pdfTitleZS: &quot;Prihláška na vzdelávanie v základnej škole - náhľad&quot;,
        pdfTitleMS: &quot;Žiadosť o prijatie dieťaťa na predprimárne vzdelávanie - náhľad&quot;,
        pdfDescription: &quot;Pilotná verzia portálu Elektronických prihlášok do škôl. Náhľad nemožno použiť ako náhradu oficiálnej prihlášky.&quot;,

        pdfVseobecneInformacieHeader: &quot;Všeobecné informácie&quot;,
        pdfVseobecneValidaciaRfo: &quot;Výsledok validácie s RFO(placeholder):&quot;,
        pdfVseobecneIdentifikatorPrihlasky: &quot;Identifikátor prihlášky:&quot;,
        pdfVseobecneDatumPodania: &quot;Dátum podania:&quot;,
        pdfVseobecneSkolskyRok: &quot;Školský rok:&quot;,
        pdfVseobecneSposobPodania: &quot;Spôsob podania:&quot;,
        pdfVseobecneStavPrihlasky: &quot;Stav prihlášky:&quot;,
        pdfVseobecneSpadovaPrihlaska: &quot;Spádová prihláška&quot;,
        pdfVseobecneDuplicitnaPrihlaska: &quot;Duplicitná prihláška&quot;,
        pdfVseobecneSurodenecNaSkole: &quot;Súrodenec na škole:&quot;,
        pdfVseobecnePoznamkaSkoly: &quot;Poznámka školy:&quot;,

        pdfDietaHeader: &quot;Podrobnosti o dieťati&quot;,
        pdfDietaMeno: &quot;Meno a priezvisko dieťaťa:&quot;,
        pdfDietaRodnePriezvisko: &quot;Rodné priezvisko:&quot;,
        pdfDietaRodneCislo: &quot;Rodné číslo:&quot;,
        pdfDietaDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfDietaPohlavie: &quot;Pohlavie:&quot;,
        pdfDietaMiestoNarodenia: &quot;Miesto narodenia:&quot;,
        pdfDietaNarodnost: &quot;Národnosť:&quot;,
        pdfDietaStatnaPrislusnost: &quot;Štátna príslušnosť:&quot;,
        pdfDietaMaterinskyJazyk: &quot;Materinský jazyk:&quot;,
        pdfDietaInyMaterinskyJazyk: &quot;Iný materinský jazyk:&quot;,
        pdfDietaAdresaTP: &quot;Adresa trvalého pobytu:&quot;,
        pdfDietaAdresaZP: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava (ak sa nezdržiava na adrese trvalého pobytu):&quot;,

        pdfSkolaHeader: &quot;Zoznam škôl&quot;,
        pdfSkolaNazov: &quot;Názov školy:&quot;,
        pdfSkolaAdresa: &quot;Adresa:&quot;,
        pdfSkolaVyucovaciJazyk: &quot;Vyučovací jazyk:&quot;,
        pdfSkolaStavPrihlasky: &quot;Stav prihlášky:&quot;,

        pdfZZHeader: &quot;Podrobnosti o zákonných zástupcoch dieťaťa&quot;,
        pdfZZMeno: &quot;Meno a priezvisko:&quot;,
        pdfZZDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfZZCisloElektronickejSchranky: &quot;Číslo elektronickej schránky:&quot;,
        pdfZZAdresaBydliska: &quot;Adresa bydliska:&quot;,
        pdfZZEmail: &quot;Email:&quot;,
        pdfZZTelefon: &quot;Telefónne číslo:&quot;,
        pdfZZ2Suhlas: &quot;Súhlas druhého zákonného zástupcu:&quot;,
        pdfZZ2DovodNezabezpeceniaSuhlasu: &quot;Dôvod nezabezpečenia súhlasu:&quot;,

        pdfPrilohyHeader: &quot;Prílohy:&quot;,

        pdfDoplnujuceInformacieHeader: &quot;Doplňujúce informácie o dieťati:&quot;,
        pdfDPDPozadovanaVychova: &quot;Požadovaná výchova:&quot;,
        pdfDPDStravovanie: &quot;Záujem o stravovanie v školskej jedálni:&quot;,
        pdfDPDDruzina: &quot;Záujem o Školský klub detí:&quot;,
        pdfDPDSVVPotreby: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním:&quot;,
        pdfDPDPopisSP: &quot;Popis znevýhodnenia / nadania:&quot;,
        pdfDPDPoznamka: &quot;Poznámka:&quot;,
        pdfDPDZiadam: &quot;Žiadam o prijatie dieťaťa na:&quot;,
        pdfDPDCelodenna: &quot;celodennú výchovu a vzdelávanie&quot;,
        pdfDPDPoldenna: &quot;poldennú výchovu a vzdelávanie&quot;,

        pdfFooter1: &quot;Generované ${dateFormatted} o ${timeFormatted} pilotnou verziou portálu Elektronických prihlášok do škôl.&quot;,
        pdfFooter2: &quot;© 2025, eprihlasky.iedu.sk&quot;,

    };




    
        
            chevron_left
        
        
            Späť
        
    
    

    
        
            
                Podrobnosti prihlášky
                
                    
                        Pridávajte dokumenty a upravujte informácie podľa potreby.
                    
                
            
            
                
                    Exportovať v PDF
                
                
                    add
                    Pridať prílohu
                
                
                    Vyžiadať prílohu
                
            
        

        
            
                info
                
                    Zaevidujte nástup dieťaťa manuálne.
                    
                        
                            Dieťa nastúpi do školy.
                            Dieťa nenastúpi do školy.
                        
                    
                
                
                    Vybrať stav
                
            

            
                check_circle

    
    
        
            
            
        
        
            
            
        
    


            
        

        
        

        
            Všeobecné informácie
            
                
                    
                        numbers
                        P-2025-90345
                    
                
                
                    
                        child_care
                        Tibor Zelený
                    
                
                
                    
                        calendar_month
                        
                            29.05.2025
                            Upraviť
                        
                        
                            
                                

    
        Dátum podania
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                            
                            
                                Späť
                                Uložiť
                            
                        
                    
                
                
                    
                        bookmark
                        2025/2026
                    
                
                
                    
                        house
                        Zaslané do škôl: 1
                    
                
                
                    
                        mail
                        Papierovo
                    
                
                
                    
                        stars
                        
                    
                        Duplicitná prihláška
                        clear
                    
                        
                            Vyberte
                            keyboard_arrow_down_rounded

                            
                                Vyberte
                            Duplicitná prihláškaSpádová prihláškaSúrodenec na škole
                        
                    
                
                
                    
                        description
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                
                
                    Nástup dieťaťa zaevidoval
                    
                
                
                    Poznámka školy
                    
                        -
                        Upraviť
                    
                    
                        
                            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/500
        
    
    


                        
                        
                            Späť
                            Uložiť
                        
                    
                
            
        

        
            
                
                    Podrobnosti o dieťati
                
                Upraviť
            
            
                
                    Meno a priezvisko dieťaťa
                    Tibor Zelený
                
                
                    Rodné priezvisko
                    Zelený
                
                
                    Rodné číslo
                    201206/0149
                
                
                    Dátum narodenia
                    06.12.2020
                
                
                    Pohlavie
                    muž
                
                
                    Miesto narodenia
                    Lučenec
                
                
                    Národnosť
                    slovenská
                
                
                    Štátna príslušnosť
                    Slovenská republika
                
                
                    Materinský jazyk
                    slovenský
                
                
                    Iný materinský jazyk
                    -
                
                
                    Adresa trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Povinné predprimárne vzdelávanie aktuálne v
                    -
                
            
        

        
            Zoznam škôl
            
                    
                        1
                        Informácie o škole č.1
                    
                    
                        Stav prihlášky
                        
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                    
                    
                        Názov školy
                        Materská škola
                    
                    
                        Adresa
                        05912, Hôrka 142
                    
                    
                        Vo vyučovacom jazyku
                        slovenský
                    
                
        

        
            
                Podrobnosti o zákonných zástupcoch dieťaťa
                Upraviť
            
            
                Informácie o zákonnom zástupcovi č.1:
                
                    
                        Meno a priezvisko
                        Emanuel Zelený
                    
                    
                        Dátum narodenia
                        22.09.1980
                    
                    
                        Číslo elektronickej schránky
                        E2125898753
                    
                    
                        Adresa bydliska
                        Gemerská cesta 18, Lučenec, Slovenská republika
                    
                    
                        Email
                        mail@mail.sk
                    
                    
                        Telefónne číslo
                        +421000222111
                    
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                
                
                Informácie o zákonnom zástupcovi č.2:
                
                    
                        Meno a priezvisko
                        Elena Hronová
                    
                    
                        Dátum narodenia
                        15.04.1985
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa bydliska
                        Námestie Svätého Egídia 45, Poprad, Slovenská republika
                    
                    
                        Email
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas druhého zákonného zástupcu
                        nie
                    
                    
                        Dôvod nezabezpečenia súhlasu
                        Zdravotný - nevie sa podpísať
                    
                

                Druhý zákonný zástupca nie je známy.
            
        

        
            
                Prílohy a doplňujúce informácie o dieťati
                Upraviť
            
            
                Prílohy

                
                    
                        article
                        Dokument.pdf / Čestné vyhlásenie zákonného zástupcu
                    
                

                

                Doplňujúce informácie o dieťati
                
                    
                        Celodenná výchova
                        áno
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        áno
                    
                    
                        Popis znevýhodnenia / nadania
                        testovací popis
                    
                    
                        Poznámka
                        -
                    
                
            
        

        
        
        
            bla bla
        
        
            Údaj sa nezhoduje s údajom rodiča v RFO.
        
        
            Údaj sa nezhoduje s údajom dieťaťa v RFO.
        

    



    

    
        assignment_turned_in
        Zaevidujte nástup dieťaťa
        

        
            

    
    
        Vyberte alternatívu:
        (nepovinné)
    
    
    
    
                        
                        Nastúpi 
                         
                        
                        Nenastúpi 
                         
    

        

        
            Späť
            Dokončiť
        
    



    
    
        
            
                
                    Items
                    
                        
                            
                                Vyhlásenie o prístupnosti
                            
                        
                        
                            
                                Kontakt na prevádzkovateľa
                            
                        
                        
                        
                            
                                Mapa stránky
                            
                        
                        
                            
                                Metodické usmernenia
                            
                        
                        
                            
                                Oznamy
                            
                        
                        
                    
                    
                        Prevádzkovateľom služby je Ministerstvo školstva, výskumu, vývoja a mládeže Slovenskej republiky.
                        
                        © 2013 - 2025, eprihlasky.iedu.sk
                    
                
                
                    
                        
                    
                
            
        
    


        
    
    
    







    



     const isLogged = true;
    const maTD = true;
    if (isLogged === true || maTD === true) {
        window.userData = {&quot;nameId&quot;:&quot;ee961ca2-3f59-4e23-b436-00a6e1075678&quot;,&quot;tokenDescriptor&quot;:&quot;c6934a87-64ec-4232-adf8-56c18b811a22:9db37aca172615ab8ec4f0d8f69cf2b74702c56b19aa8b99a5ec826006b4f122&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930571647&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgSUQ9Il9mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI19mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT5yN055bFVTR1NGMlpkVnVHNHlEVTN2eTdJTDFHV1E3dVlmUk5mQzBwbEo4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5VRVNiZDlDcTMzV3hta1hYU3dlaGpkcnNaNjlUUDJ0YnNlQWRQVnI0WFc3WWorRDRmQlJOQnhxbXZ2QTlYRnhYYWJTZmlpeGd4SFZNNjJ1SHVxeXZDNkh5VUpPeGZ5UHVhTlVPWXVlR3lzWjl4ZWczR1dIaHljN1JWRW5qWmloQVo2VkQ0UC9nWHM4YWdBRUR5Qms4dXlPY1VrVVZ6alFDa2xTMENhemV3eHdGMHRqSFhuTWlURTFBeU1EZTBFQUFMcFRrcWgyZy9zS0pzb1F6d0tjbzlZb1MvKzFPdHhzWWpsSGtkbjcvUWk5WmVabUc1eENKaGhBaHNnV2ZoZ3gxdHV4enZvbk43L0c5ejhlQWpHNTVyU0tpU21KNkYwMTBBUVA4bWxQVVB0R3ZFS09UVEFNWCtzeDYvcDBDeHd4dHF5RUN1V2FWT29aY2pCU1pFajkxL0E9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfYWJmNGQyYTMtMzQ1Yy00ZTJmLWIwNTUtNjQyOTllZmZmNWM1IiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmVlOTYxY2EyLTNmNTktNGUyMy1iNDM2LTAwYTZlMTA3NTY3ODwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDExOjMyOjExLjc3MloiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMToxNzoxMS43NzJaIiBTZXNzaW9uSW5kZXg9ImM2OTM0YTg3LTY0ZWMtNDIzMi1hZGY4LTU2YzE4YjgxMWEyMjo5ZGIzN2FjYTE3MjYxNWFiOGVjNGYwZDhmNjljZjJiNzQ3MDJjNTZiMTlhYThiOTlhNWVjODI2MDA2YjRmMTIyIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkthcm9sw61uYTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvUHJpZXp2aXNrbyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkxpZXRhasO6Y2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MDkuMDMuMTk5MDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5ERkVFMUJCNi0wMzhGLTQ2MDMtQUYxQi04RkU2QjA5MTIwOTU8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MTY0Nzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcxNjQ3PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930571647&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;birthNumber&quot;:null,&quot;genderCode&quot;:&quot;2&quot;,&quot;dateOfBirth&quot;:&quot;1990-03-09&quot;,&quot;name&quot;:&quot;Karolína&quot;,&quot;surname&quot;:&quot;Lietajúca&quot;,&quot;subjectGuid&quot;:&quot;acc1dadc-299a-4977-94bd-0b5e15e77a6b&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;identifikatorVIAM&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910012370&quot;,&quot;vybrataSaszNazov&quot;:&quot;Materská škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910012370&quot;,&quot;nazov&quot;:&quot;Materská škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-10&quot;}]}};
    }
    window.env = &quot;ZakTest&quot;;


    function loadJsCSHTML() {
        dtc.globalTranslations = {
            cookies: {
                title: &quot;Potvrdenie súhlasu s cookies&quot;,
                description: &quot;S cieľom zabezpečiť správne fungovanie tejto webovej lokality, ukladáme v niektorých prípadoch na vašom zariadení malé dátové súbory, tzv. cookie.&lt;br>Tieto súbory nám pomáhajú poskytovať kvalitnejšie služby, čím vám umožňujeme pohodlnejšie využívanie stránok.&lt;br>Taktiež používame analytické súbory, akceptovaním súhlasíte s ich používaním.&quot;,
                button: &quot;Súhlasím&quot;,
                buttonSecondary: &quot;Nesúhlasím&quot;
            },
            overlay: {
                closeButton: &quot;Zatvoriť&quot;,
                yes: &quot;Áno&quot;,
                no: &quot;Nie&quot;,
                ok: &quot;OK&quot;
            },
            errorSummaryBox: {
                ajaxError: {
                    title: &quot;Systémová chyba&quot;
                },
                title: &quot;Nasledujúce polia neboli vyplnené správne:&quot;,
                description: &quot;&quot;,
            },
            systemError: {
                systemError: &quot; , &quot;'&quot; , &quot;Systémová chyba&quot; , &quot;'&quot; , &quot;,
                server: &quot; , &quot;'&quot; , &quot;Došlo k chybe pri komunikácii so serverom. V prípade pretrvávania chyby kontaktujte zákaznícku podporu.&quot; , &quot;'&quot; , &quot;
            },
            calendar: {
                days: [
                    &quot; , &quot;'&quot; , &quot;Nedeľa&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Pondelok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Utorok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Streda&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Štvrtok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Piatok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Sobota&quot; , &quot;'&quot; , &quot;,
                ],
                months: [
                    &quot; , &quot;'&quot; , &quot;Január&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Február&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Marec&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Apríl&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Máj&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Jún&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Júl&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;August&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;September&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Október&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;November&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;December&quot; , &quot;'&quot; , &quot;,
                ]
            }
        };

        dtc.links = {
            home: &quot;/&quot;,
            pomocAPodpora: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora&quot; , &quot;'&quot; , &quot;,
            zakladneInformacie: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Zakladne-informacie&quot; , &quot;'&quot; , &quot;,
            akoPodatPrihlaskuNaZS: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Ako-podat-prihlasku-na-ZS&quot; , &quot;'&quot; , &quot;,
            akoPodatPrihlaskuNaMS: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Ako-podat-prihlasku-na-MS&quot; , &quot;'&quot; , &quot;,
            mojProfil: &quot; , &quot;'&quot; , &quot;/Moj-profil&quot; , &quot;'&quot; , &quot;,
            dieta: &quot; , &quot;'&quot; , &quot;/Dieta&quot; , &quot;'&quot; , &quot;,
            nastavenie: &quot; , &quot;'&quot; , &quot;/Nastavenie&quot; , &quot;'&quot; , &quot;,
            prihlaska: &quot; , &quot;'&quot; , &quot;/Prihlaska&quot; , &quot;'&quot; , &quot;,
            spatnaVazba: &quot; , &quot;'&quot; , &quot;/Spatna-vazba&quot; , &quot;'&quot; , &quot;,
            prihlaskaOdoslana: &quot; , &quot;'&quot; , &quot;/Prihlaska-odoslana&quot; , &quot;'&quot; , &quot;,
            oznamy: &quot; , &quot;'&quot; , &quot;/Oznamy&quot; , &quot;'&quot; , &quot;,
            mapaStranok: &quot; , &quot;'&quot; , &quot;/Mapa-stranok&quot; , &quot;'&quot; , &quot;,
            selectSaszCase: &quot; , &quot;'&quot; , &quot;/SelectSaszCase&quot; , &quot;'&quot; , &quot;,
            ziadost: &quot; , &quot;'&quot; , &quot;/Ziadost&quot; , &quot;'&quot; , &quot;,
            riaditel: &quot; , &quot;'&quot; , &quot;/Riaditel&quot; , &quot;'&quot; , &quot;,
            rozhodnutia: &quot; , &quot;'&quot; , &quot;/Riaditel&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;?Rozhodnutia&quot; , &quot;'&quot; , &quot;,
            selectSaszCase: &quot; , &quot;'&quot; , &quot;/SelectSaszCase&quot; , &quot;'&quot; , &quot;,
            &quot;401error&quot;: &quot; , &quot;'&quot; , &quot;/Logout?returnUrl=/Error?code=401&quot; , &quot;'&quot; , &quot;,
            vydatRozhodnutia: &quot; , &quot;'&quot; , &quot;/Riaditel/Vydat-rozhodnutia&quot; , &quot;'&quot; , &quot;
        };
    }

    

    

/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]&quot;) or . = concat(&quot;
    
    

        
            Testovacie prostredie
        
        
            
                error
                Dôležité: Tento portál je momentálne v pilotnej verzii a je dostupný len pre vybrané školy. Viac informácií.
            
        
    
        
            
                
                    
                        

                            
                                Oficiálna stránka

                                
                                    e-Gov
                                    verejnej správy SR
                                    arrow_drop_down
                                
                            
                            
                                
                                    
                                        Doména gov.sk je oficiálna
                                        Toto je oficiálna webová stránka orgánu verejnej moci Slovenskej republiky. Oficiálne stránky využívajú najmä doménu gov.sk. Odkazy na jednotlivé webové sídla orgánov verejnej moci nájdete na tomto odkaze.
                                    
                                    
                                        Táto stránka je zabezpečená
                                        Buďte pozorní a vždy sa uistite, že zdieľate informácie iba cez zabezpečenú webovú stránku verejnej správy SR. Zabezpečená stránka vždy začína https:// pred názvom domény webového sídla.
                                    
                                
                            
                        
                        
                            
                                slovenčina
                                arrow_drop_down
                            
                            
                                        
                                            
                                                slovenčina
                                            
                                        
                            
                        
                    
                
            

            
                
                    
                        landscape
                    
                    
                        Elektronické prihlášky do škôl
                    
                
                
                    
                        
                            Menu
                            
                                menuicon
                            
                        
                    
                            
                                
                                    
                                        KL
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const controlSettingsPodrobnostiPrihlasky = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        validovaneSNesuladom2: &quot;Validované s nesúladom:&quot;,
        validovaneSNesuladom3: &quot;Nevalidované voči RFO&quot;,
        validovaneSNesuladom4: &quot;Prebieha overenie údajov&quot;,
        zaslaneDoSkol: &quot;Zaslané do škôl:&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaStavPrihlasky: &quot;Stav prihlášky&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8086&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zadajte dátum v aktuálnom roku menší alebo rovný aktuálnemu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8087&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Informáciu o nástupe dieťaťa ste úspešne zaevidovali.&quot; , &quot;'&quot; , &quot;,
        datumPodaniaLabel: &quot;Dátum podania&quot;,
        udajSaNezhodujeRFOPopup: &quot;Údaj sa nezhoduje s údajom rodiča v RFO.&quot;,
        udajSaNezhodujeRFODietaPopup: &quot;Údaj sa nezhoduje s údajom dieťaťa v RFO.&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        zaevidujteNastupDescription: &quot;Pre zaevidovanie informácie, či dieťa nastúpi alebo nenastúpi do školy, je potrebné vybrať jednu z možností nižšie. V prípade výberu možnosti Nastúpi, bude pre všetky ostatné školy, kde dieťa prijali, automaticky označené, že nenastúpi. Nástup je možné zaevidovať alebo zmeniť do $datum$.&quot;,
        podlaRIS: &quot;podľa RIS&quot;,
        nieJeKDispoziciiZRIS: &quot;(Tento údaj nie je k dispozícii z RIS.)&quot;,

        vyberAlternativuRadio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte alternatívu:&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;Nastúpi&quot;,
                    value: &quot;1&quot;,
                },
                {
                    label: &quot;Nenastúpi&quot;,
                    value: &quot;0&quot;,
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ],

        },


        // pdf
        pdfTitleZS: &quot;Prihláška na vzdelávanie v základnej škole - náhľad&quot;,
        pdfTitleMS: &quot;Žiadosť o prijatie dieťaťa na predprimárne vzdelávanie - náhľad&quot;,
        pdfDescription: &quot;Pilotná verzia portálu Elektronických prihlášok do škôl. Náhľad nemožno použiť ako náhradu oficiálnej prihlášky.&quot;,

        pdfVseobecneInformacieHeader: &quot;Všeobecné informácie&quot;,
        pdfVseobecneValidaciaRfo: &quot;Výsledok validácie s RFO(placeholder):&quot;,
        pdfVseobecneIdentifikatorPrihlasky: &quot;Identifikátor prihlášky:&quot;,
        pdfVseobecneDatumPodania: &quot;Dátum podania:&quot;,
        pdfVseobecneSkolskyRok: &quot;Školský rok:&quot;,
        pdfVseobecneSposobPodania: &quot;Spôsob podania:&quot;,
        pdfVseobecneStavPrihlasky: &quot;Stav prihlášky:&quot;,
        pdfVseobecneSpadovaPrihlaska: &quot;Spádová prihláška&quot;,
        pdfVseobecneDuplicitnaPrihlaska: &quot;Duplicitná prihláška&quot;,
        pdfVseobecneSurodenecNaSkole: &quot;Súrodenec na škole:&quot;,
        pdfVseobecnePoznamkaSkoly: &quot;Poznámka školy:&quot;,

        pdfDietaHeader: &quot;Podrobnosti o dieťati&quot;,
        pdfDietaMeno: &quot;Meno a priezvisko dieťaťa:&quot;,
        pdfDietaRodnePriezvisko: &quot;Rodné priezvisko:&quot;,
        pdfDietaRodneCislo: &quot;Rodné číslo:&quot;,
        pdfDietaDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfDietaPohlavie: &quot;Pohlavie:&quot;,
        pdfDietaMiestoNarodenia: &quot;Miesto narodenia:&quot;,
        pdfDietaNarodnost: &quot;Národnosť:&quot;,
        pdfDietaStatnaPrislusnost: &quot;Štátna príslušnosť:&quot;,
        pdfDietaMaterinskyJazyk: &quot;Materinský jazyk:&quot;,
        pdfDietaInyMaterinskyJazyk: &quot;Iný materinský jazyk:&quot;,
        pdfDietaAdresaTP: &quot;Adresa trvalého pobytu:&quot;,
        pdfDietaAdresaZP: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava (ak sa nezdržiava na adrese trvalého pobytu):&quot;,

        pdfSkolaHeader: &quot;Zoznam škôl&quot;,
        pdfSkolaNazov: &quot;Názov školy:&quot;,
        pdfSkolaAdresa: &quot;Adresa:&quot;,
        pdfSkolaVyucovaciJazyk: &quot;Vyučovací jazyk:&quot;,
        pdfSkolaStavPrihlasky: &quot;Stav prihlášky:&quot;,

        pdfZZHeader: &quot;Podrobnosti o zákonných zástupcoch dieťaťa&quot;,
        pdfZZMeno: &quot;Meno a priezvisko:&quot;,
        pdfZZDatumNarodenia: &quot;Dátum narodenia:&quot;,
        pdfZZCisloElektronickejSchranky: &quot;Číslo elektronickej schránky:&quot;,
        pdfZZAdresaBydliska: &quot;Adresa bydliska:&quot;,
        pdfZZEmail: &quot;Email:&quot;,
        pdfZZTelefon: &quot;Telefónne číslo:&quot;,
        pdfZZ2Suhlas: &quot;Súhlas druhého zákonného zástupcu:&quot;,
        pdfZZ2DovodNezabezpeceniaSuhlasu: &quot;Dôvod nezabezpečenia súhlasu:&quot;,

        pdfPrilohyHeader: &quot;Prílohy:&quot;,

        pdfDoplnujuceInformacieHeader: &quot;Doplňujúce informácie o dieťati:&quot;,
        pdfDPDPozadovanaVychova: &quot;Požadovaná výchova:&quot;,
        pdfDPDStravovanie: &quot;Záujem o stravovanie v školskej jedálni:&quot;,
        pdfDPDDruzina: &quot;Záujem o Školský klub detí:&quot;,
        pdfDPDSVVPotreby: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním:&quot;,
        pdfDPDPopisSP: &quot;Popis znevýhodnenia / nadania:&quot;,
        pdfDPDPoznamka: &quot;Poznámka:&quot;,
        pdfDPDZiadam: &quot;Žiadam o prijatie dieťaťa na:&quot;,
        pdfDPDCelodenna: &quot;celodennú výchovu a vzdelávanie&quot;,
        pdfDPDPoldenna: &quot;poldennú výchovu a vzdelávanie&quot;,

        pdfFooter1: &quot;Generované ${dateFormatted} o ${timeFormatted} pilotnou verziou portálu Elektronických prihlášok do škôl.&quot;,
        pdfFooter2: &quot;© 2025, eprihlasky.iedu.sk&quot;,

    };




    
        
            chevron_left
        
        
            Späť
        
    
    

    
        
            
                Podrobnosti prihlášky
                
                    
                        Pridávajte dokumenty a upravujte informácie podľa potreby.
                    
                
            
            
                
                    Exportovať v PDF
                
                
                    add
                    Pridať prílohu
                
                
                    Vyžiadať prílohu
                
            
        

        
            
                info
                
                    Zaevidujte nástup dieťaťa manuálne.
                    
                        
                            Dieťa nastúpi do školy.
                            Dieťa nenastúpi do školy.
                        
                    
                
                
                    Vybrať stav
                
            

            
                check_circle

    
    
        
            
            
        
        
            
            
        
    


            
        

        
        

        
            Všeobecné informácie
            
                
                    
                        numbers
                        P-2025-90345
                    
                
                
                    
                        child_care
                        Tibor Zelený
                    
                
                
                    
                        calendar_month
                        
                            29.05.2025
                            Upraviť
                        
                        
                            
                                

    
        Dátum podania
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                            
                            
                                Späť
                                Uložiť
                            
                        
                    
                
                
                    
                        bookmark
                        2025/2026
                    
                
                
                    
                        house
                        Zaslané do škôl: 1
                    
                
                
                    
                        mail
                        Papierovo
                    
                
                
                    
                        stars
                        
                    
                        Duplicitná prihláška
                        clear
                    
                        
                            Vyberte
                            keyboard_arrow_down_rounded

                            
                                Vyberte
                            Duplicitná prihláškaSpádová prihláškaSúrodenec na škole
                        
                    
                
                
                    
                        description
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                
                
                    Nástup dieťaťa zaevidoval
                    
                
                
                    Poznámka školy
                    
                        -
                        Upraviť
                    
                    
                        
                            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/500
        
    
    


                        
                        
                            Späť
                            Uložiť
                        
                    
                
            
        

        
            
                
                    Podrobnosti o dieťati
                
                Upraviť
            
            
                
                    Meno a priezvisko dieťaťa
                    Tibor Zelený
                
                
                    Rodné priezvisko
                    Zelený
                
                
                    Rodné číslo
                    201206/0149
                
                
                    Dátum narodenia
                    06.12.2020
                
                
                    Pohlavie
                    muž
                
                
                    Miesto narodenia
                    Lučenec
                
                
                    Národnosť
                    slovenská
                
                
                    Štátna príslušnosť
                    Slovenská republika
                
                
                    Materinský jazyk
                    slovenský
                
                
                    Iný materinský jazyk
                    -
                
                
                    Adresa trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                    Gemerská cesta 18/46, 99988, Lučenec, Slovenská republika
                
                
                    Povinné predprimárne vzdelávanie aktuálne v
                    -
                
            
        

        
            Zoznam škôl
            
                    
                        1
                        Informácie o škole č.1
                    
                    
                        Stav prihlášky
                        
                        
                            V spracovaní
                            keyboard_arrow_down_rounded

                            
                                V spracovaní
                            Návrh na prijatieNávrh na neprijatie
                        
                    
                    
                    
                        Názov školy
                        Materská škola
                    
                    
                        Adresa
                        05912, Hôrka 142
                    
                    
                        Vo vyučovacom jazyku
                        slovenský
                    
                
        

        
            
                Podrobnosti o zákonných zástupcoch dieťaťa
                Upraviť
            
            
                Informácie o zákonnom zástupcovi č.1:
                
                    
                        Meno a priezvisko
                        Emanuel Zelený
                    
                    
                        Dátum narodenia
                        22.09.1980
                    
                    
                        Číslo elektronickej schránky
                        E2125898753
                    
                    
                        Adresa bydliska
                        Gemerská cesta 18, Lučenec, Slovenská republika
                    
                    
                        Email
                        mail@mail.sk
                    
                    
                        Telefónne číslo
                        +421000222111
                    
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                
                
                Informácie o zákonnom zástupcovi č.2:
                
                    
                        Meno a priezvisko
                        Elena Hronová
                    
                    
                        Dátum narodenia
                        15.04.1985
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa bydliska
                        Námestie Svätého Egídia 45, Poprad, Slovenská republika
                    
                    
                        Email
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas druhého zákonného zástupcu
                        nie
                    
                    
                        Dôvod nezabezpečenia súhlasu
                        Zdravotný - nevie sa podpísať
                    
                

                Druhý zákonný zástupca nie je známy.
            
        

        
            
                Prílohy a doplňujúce informácie o dieťati
                Upraviť
            
            
                Prílohy

                
                    
                        article
                        Dokument.pdf / Čestné vyhlásenie zákonného zástupcu
                    
                

                

                Doplňujúce informácie o dieťati
                
                    
                        Celodenná výchova
                        áno
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        áno
                    
                    
                        Popis znevýhodnenia / nadania
                        testovací popis
                    
                    
                        Poznámka
                        -
                    
                
            
        

        
        
        
            bla bla
        
        
            Údaj sa nezhoduje s údajom rodiča v RFO.
        
        
            Údaj sa nezhoduje s údajom dieťaťa v RFO.
        

    



    

    
        assignment_turned_in
        Zaevidujte nástup dieťaťa
        

        
            

    
    
        Vyberte alternatívu:
        (nepovinné)
    
    
    
    
                        
                        Nastúpi 
                         
                        
                        Nenastúpi 
                         
    

        

        
            Späť
            Dokončiť
        
    



    
    
        
            
                
                    Items
                    
                        
                            
                                Vyhlásenie o prístupnosti
                            
                        
                        
                            
                                Kontakt na prevádzkovateľa
                            
                        
                        
                        
                            
                                Mapa stránky
                            
                        
                        
                            
                                Metodické usmernenia
                            
                        
                        
                            
                                Oznamy
                            
                        
                        
                    
                    
                        Prevádzkovateľom služby je Ministerstvo školstva, výskumu, vývoja a mládeže Slovenskej republiky.
                        
                        © 2013 - 2025, eprihlasky.iedu.sk
                    
                
                
                    
                        
                    
                
            
        
    


        
    
    
    







    



     const isLogged = true;
    const maTD = true;
    if (isLogged === true || maTD === true) {
        window.userData = {&quot;nameId&quot;:&quot;ee961ca2-3f59-4e23-b436-00a6e1075678&quot;,&quot;tokenDescriptor&quot;:&quot;c6934a87-64ec-4232-adf8-56c18b811a22:9db37aca172615ab8ec4f0d8f69cf2b74702c56b19aa8b99a5ec826006b4f122&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930571647&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgSUQ9Il9mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI19mNzliZDVjNi0xYzU5LTRlNjktODM1MC1mZGFiMTUxYmI5YTkiPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT5yN055bFVTR1NGMlpkVnVHNHlEVTN2eTdJTDFHV1E3dVlmUk5mQzBwbEo4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5VRVNiZDlDcTMzV3hta1hYU3dlaGpkcnNaNjlUUDJ0YnNlQWRQVnI0WFc3WWorRDRmQlJOQnhxbXZ2QTlYRnhYYWJTZmlpeGd4SFZNNjJ1SHVxeXZDNkh5VUpPeGZ5UHVhTlVPWXVlR3lzWjl4ZWczR1dIaHljN1JWRW5qWmloQVo2VkQ0UC9nWHM4YWdBRUR5Qms4dXlPY1VrVVZ6alFDa2xTMENhemV3eHdGMHRqSFhuTWlURTFBeU1EZTBFQUFMcFRrcWgyZy9zS0pzb1F6d0tjbzlZb1MvKzFPdHhzWWpsSGtkbjcvUWk5WmVabUc1eENKaGhBaHNnV2ZoZ3gxdHV4enZvbk43L0c5ejhlQWpHNTVyU0tpU21KNkYwMTBBUVA4bWxQVVB0R3ZFS09UVEFNWCtzeDYvcDBDeHd4dHF5RUN1V2FWT29aY2pCU1pFajkxL0E9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfYWJmNGQyYTMtMzQ1Yy00ZTJmLWIwNTUtNjQyOTllZmZmNWM1IiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmVlOTYxY2EyLTNmNTktNGUyMy1iNDM2LTAwYTZlMTA3NTY3ODwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2Y0NDI2MjA3LTYzZmItNGMwNC1hZDM3LWRmMmM3OTZlODYyNCIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTE6MTc6MTEuNzcyWiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDExOjMyOjExLjc3MloiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMToxNzoxMS43NzJaIiBTZXNzaW9uSW5kZXg9ImM2OTM0YTg3LTY0ZWMtNDIzMi1hZGY4LTU2YzE4YjgxMWEyMjo5ZGIzN2FjYTE3MjYxNWFiOGVjNGYwZDhmNjljZjJiNzQ3MDJjNTZiMTlhYThiOTlhNWVjODI2MDA2YjRmMTIyIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAzOjE3OjExLjY3MFoiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkthcm9sw61uYTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvUHJpZXp2aXNrbyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkxpZXRhasO6Y2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MDkuMDMuMTk5MDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5ERkVFMUJCNi0wMzhGLTQ2MDMtQUYxQi04RkU2QjA5MTIwOTU8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MTY0Nzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcxNjQ3PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930571647&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;birthNumber&quot;:null,&quot;genderCode&quot;:&quot;2&quot;,&quot;dateOfBirth&quot;:&quot;1990-03-09&quot;,&quot;name&quot;:&quot;Karolína&quot;,&quot;surname&quot;:&quot;Lietajúca&quot;,&quot;subjectGuid&quot;:&quot;acc1dadc-299a-4977-94bd-0b5e15e77a6b&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;de7f96cd-ec83-47fa-85b7-cf13778d1f78&quot;,&quot;identifikatorVIAM&quot;:&quot;DFEE1BB6-038F-4603-AF1B-8FE6B0912095&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910012370&quot;,&quot;vybrataSaszNazov&quot;:&quot;Materská škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910012370&quot;,&quot;nazov&quot;:&quot;Materská škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-10&quot;}]}};
    }
    window.env = &quot;ZakTest&quot;;


    function loadJsCSHTML() {
        dtc.globalTranslations = {
            cookies: {
                title: &quot;Potvrdenie súhlasu s cookies&quot;,
                description: &quot;S cieľom zabezpečiť správne fungovanie tejto webovej lokality, ukladáme v niektorých prípadoch na vašom zariadení malé dátové súbory, tzv. cookie.&lt;br>Tieto súbory nám pomáhajú poskytovať kvalitnejšie služby, čím vám umožňujeme pohodlnejšie využívanie stránok.&lt;br>Taktiež používame analytické súbory, akceptovaním súhlasíte s ich používaním.&quot;,
                button: &quot;Súhlasím&quot;,
                buttonSecondary: &quot;Nesúhlasím&quot;
            },
            overlay: {
                closeButton: &quot;Zatvoriť&quot;,
                yes: &quot;Áno&quot;,
                no: &quot;Nie&quot;,
                ok: &quot;OK&quot;
            },
            errorSummaryBox: {
                ajaxError: {
                    title: &quot;Systémová chyba&quot;
                },
                title: &quot;Nasledujúce polia neboli vyplnené správne:&quot;,
                description: &quot;&quot;,
            },
            systemError: {
                systemError: &quot; , &quot;'&quot; , &quot;Systémová chyba&quot; , &quot;'&quot; , &quot;,
                server: &quot; , &quot;'&quot; , &quot;Došlo k chybe pri komunikácii so serverom. V prípade pretrvávania chyby kontaktujte zákaznícku podporu.&quot; , &quot;'&quot; , &quot;
            },
            calendar: {
                days: [
                    &quot; , &quot;'&quot; , &quot;Nedeľa&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Pondelok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Utorok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Streda&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Štvrtok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Piatok&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Sobota&quot; , &quot;'&quot; , &quot;,
                ],
                months: [
                    &quot; , &quot;'&quot; , &quot;Január&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Február&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Marec&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Apríl&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Máj&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Jún&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Júl&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;August&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;September&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;Október&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;November&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;December&quot; , &quot;'&quot; , &quot;,
                ]
            }
        };

        dtc.links = {
            home: &quot;/&quot;,
            pomocAPodpora: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora&quot; , &quot;'&quot; , &quot;,
            zakladneInformacie: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Zakladne-informacie&quot; , &quot;'&quot; , &quot;,
            akoPodatPrihlaskuNaZS: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Ako-podat-prihlasku-na-ZS&quot; , &quot;'&quot; , &quot;,
            akoPodatPrihlaskuNaMS: &quot; , &quot;'&quot; , &quot;/Pomoc-a-podpora/Ako-podat-prihlasku-na-MS&quot; , &quot;'&quot; , &quot;,
            mojProfil: &quot; , &quot;'&quot; , &quot;/Moj-profil&quot; , &quot;'&quot; , &quot;,
            dieta: &quot; , &quot;'&quot; , &quot;/Dieta&quot; , &quot;'&quot; , &quot;,
            nastavenie: &quot; , &quot;'&quot; , &quot;/Nastavenie&quot; , &quot;'&quot; , &quot;,
            prihlaska: &quot; , &quot;'&quot; , &quot;/Prihlaska&quot; , &quot;'&quot; , &quot;,
            spatnaVazba: &quot; , &quot;'&quot; , &quot;/Spatna-vazba&quot; , &quot;'&quot; , &quot;,
            prihlaskaOdoslana: &quot; , &quot;'&quot; , &quot;/Prihlaska-odoslana&quot; , &quot;'&quot; , &quot;,
            oznamy: &quot; , &quot;'&quot; , &quot;/Oznamy&quot; , &quot;'&quot; , &quot;,
            mapaStranok: &quot; , &quot;'&quot; , &quot;/Mapa-stranok&quot; , &quot;'&quot; , &quot;,
            selectSaszCase: &quot; , &quot;'&quot; , &quot;/SelectSaszCase&quot; , &quot;'&quot; , &quot;,
            ziadost: &quot; , &quot;'&quot; , &quot;/Ziadost&quot; , &quot;'&quot; , &quot;,
            riaditel: &quot; , &quot;'&quot; , &quot;/Riaditel&quot; , &quot;'&quot; , &quot;,
            rozhodnutia: &quot; , &quot;'&quot; , &quot;/Riaditel&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;?Rozhodnutia&quot; , &quot;'&quot; , &quot;,
            selectSaszCase: &quot; , &quot;'&quot; , &quot;/SelectSaszCase&quot; , &quot;'&quot; , &quot;,
            &quot;401error&quot;: &quot; , &quot;'&quot; , &quot;/Logout?returnUrl=/Error?code=401&quot; , &quot;'&quot; , &quot;,
            vydatRozhodnutia: &quot; , &quot;'&quot; , &quot;/Riaditel/Vydat-rozhodnutia&quot; , &quot;'&quot; , &quot;
        };
    }

    

    

/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]&quot;))]</value>
      <webElementGuid>dbf58e12-db61-442f-b14c-48857fb2302e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
