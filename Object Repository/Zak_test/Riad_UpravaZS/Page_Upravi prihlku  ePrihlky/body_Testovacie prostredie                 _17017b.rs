<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Testovacie prostredie                 _17017b</name>
   <tag></tag>
   <elementGuidId>eb4228fa-3af3-401c-b390-b607e57d2a7f</elementGuidId>
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
      <webElementGuid>46ceed74-ed51-4449-89c5-46959b99d414</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-frontend-supported</value>
      <webElementGuid>393c006a-d231-40ab-970b-70c0230a7ff7</webElementGuid>
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
                            
                        
                    
                            
                                
                                    
                                        JN
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const pageTextsDieta = {
        '8020': &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        '8021': &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        '8042': 'Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.',
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
        },
        krstneMeno: {
            label: &quot;Krstné meno&quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        priezvisko: {
            label: &quot;Priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        rodnePriezvisko: {
            label: &quot;Rodné priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        datumNarodenia: {
            label: &quot;Dátum narodenia&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        pohlavieRadio: {
            label: &quot;Pohlavie&quot;,
            options: [
                {
                    label: &quot;Muž&quot;,
                    value: '1'
                },
                {
                    label: &quot;Žena&quot;,
                    value: '2'
                }
            ],
            direction: 'row'
        },
        miestoNarodenia: {
            label: &quot;Miesto narodenia&quot;,
            required: true,
            regexError: &quot;Zadajte miesto narodenia.&quot;,
        },
        narodnost: {
            label: &quot;Národnosť&quot;,
            required: true
        },
        statnaPrislusnost: {
            label: &quot;Štátna príslušnosť&quot;,
            selectError: &quot;Vyhľadajte štát po zadaní prvých 3 znakov a vyberte štát zo zoznamu zobrazených štátov!&quot;,
            required: true
        },
        materinskyJazyk: {
            label: &quot;Materinský jazyk&quot;,
            required: true
        },
        inyMaterinskyJazyk: {
            label: &quot;Iný materinský jazyk&quot;,
            required: false
        },
        trvalyPobytRadio: {
            label: &quot;Adresa trvalého pobytu dieťaťa&quot;,
            required: true,
            direction: 'column',
            options: [
                {
                    label: &quot;#####&quot;,
                    value: '1'
                },
                {
                    label: &quot;Iná adresa trvalého pobytu&quot;,
                    value: '2'
                }
            ]
        },
        zvycajnaAdresaradio: {
            label: &quot;Adresa miesta, kde dieťa zvyčajne žije&quot;,
            hint: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.&quot;,
            required: true,
            direction: 'column',
            options: [
                {
                    label: 'Zhodná s adresou trvalého pobytu dieťaťa',
                    value: '1'
                },
                {
                    label: &quot;Iná adresa&quot;,
                    value: '2'
                }
            ],
            direction: 'column'
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90346.01
    

    
        Podrobnosti o dieťati
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
            
                Všeobecné informácie
                
                    

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Krstné meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          falseClear date    

                
                
                    

    
    
        Pohlavie
        (nepovinné)
    
    
    
    
                        
                        Muž 
                         
                        
                        Žena 
                         
    

                
            

            
                Štátna príslušnosť a jazyk
                
                    

    
        Miesto narodenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Národnosť
        (nepovinné)
    
    
    
        
        slovenská
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
abchádzskyafanoromskýafarskýafgánskyajmarskýalbánskyamharskýanglickýarabskýarménskyasamskýazerbajdžanskýBarcianskybaskickýbaškirskýbengálskybhútanskýbieloruskýbiharskýbirmskýbislamskýbretónskybulharskýčeskýceylonský /singalezština/chorvátskyčínskydánskyeskimáckyesperantoestónskyfaerskýfidžijskýfínskyflámskyfrancúzskyfrízskygréckygrónskygruzínskyguaranígudžarátskyhaličskýhaustskýhebrejskýhindustánskyholandskýindonézskyinterliguaskýinupiakskýinýiný (mŕtvy jazyk)írskyislandskýjaponskýjavanskýjidišskýjuhoafrická holandštinakambodžskýkanadskýkašmírskykatalánskykazachskýkinjarwandskýkirgizskýkirundikórejskýkorzickýkurdskýlaoskýlatinskýlingalskýlitovskýlotyšskýmacedónskymadagaskarskýmaďarskýmalabarskýmalajzskýmaltézskyMaori /jazyk novozélandskej rasy/maráthskýmohamedánska hindustuuštinamongolskýmultanský /moldavský/nautskýnemeckýnepálskynesleduje sanórskyocitanskýorjanskýpandžábskyperzskýpoľskýportugalskýrhaetorománskyrómskyrumunskýrusínskyruskýsamojskýsangskýsanskritskýsindhinskýsiswatskýškótskyslovenskýslovinskýsomálskyšonskýsothošpanielskysrbochorvátskysrbskýstwanskýsuahelisudánskyšvédskytadžickýtagalskýtalianskytamilskýťanský (ťvi)tatarskýtelugskýthajskýtibetskýtigrinskýtongskýtureckýturkménskyudmurský (voťačský)ukrajinskýuzbeckývietnamskýwaleskýwelskýwolofskýxhosayorubský (yoruba)židovskýzulu /kafr/

                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
český

                

            

            
                Trvalý pobyt
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window['adresaTPControlSettings'] = {
        adresaTPKrajina: {
            label: 'Krajina',
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!'
                }
            ]
        },
        adresaTPObec: {
            label: 'Obec',
            required: true,
            attributes: {
                maxLength: '255'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!'
                }
            ]
        },
        adresaTPUlica: {
            label: 'Ulica',
                required: false,
                    attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!'
                }
            ]
        },
        adresaTPUlicaReq: {
            label: 'Ulica',
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!'
                }
            ]
        },
        adresaTPSupisneCislo: {
            label: 'Súpisné číslo',
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: '10'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaTPOrientacneCislo: {
            label: 'Orientačné číslo',
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: '20'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaTPPSC: {
            label: 'PSČ',
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: '10'
            },
            validators: [
               {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaTPAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\'\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
                    

    
    
        Adresa miesta, kde dieťa zvyčajne žije
        (nepovinné)
    
    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window['adresaZAControlSettings'] = {
        adresaZAKrajina: {
            label: 'Krajina',
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!'
                }
            ]
        },
        adresaZAObec: {
            label: 'Obec',
            required: true,
            attributes: {
                maxLength: '255'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!'
                }
            ]
        },
        adresaZAUlica: {
            label: 'Ulica',
                required: false,
                    attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!'
                }
            ]
        },
        adresaZAUlicaReq: {
            label: 'Ulica',
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'select',
                    message: 'Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!'
                }
            ]
        },
        adresaZASupisneCislo: {
            label: 'Súpisné číslo',
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: '10'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaZAOrientacneCislo: {
            label: 'Orientačné číslo',
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: '20'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaZAPSC: {
            label: 'PSČ',
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: '10'
            },
            validators: [
               {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaZAAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: '100'
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\'\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
            

            
                Predprimárne vzdelávanie
                
                    

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
            

        

    
        Zrušiť
        
            Uložiť a späť na prihlášku
        
    



    
    
        
            
                
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
        window.userData = {&quot;nameId&quot;:&quot;e0ab0dbb-5e21-4d45-8c74-4100e903e979&quot;,&quot;tokenDescriptor&quot;:&quot;e015c0bf-4bfc-4247-8544-97806a6fdc38:3d83b976c8ab0f1bdbaaa8213aaa037c52eef0a820b097c9f8f7ac908e4e70bb&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930570810&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgSUQ9Il84M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI184M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT43Wi9sckI5ZG9sUWNoQUZzdi9BVXc4ZTNkUzFzYzlSc250cmlSSS9lWkg4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5ESmtDQmVFblVRc3NTVkdoQXFobzEwMzgwSzU1cllLR2ZSZloyZmhUYnFFQVVZeUpwd3VJNHlHVG5FR05QbTkyWkxkOXBBTmgrZFBSSll4V2xTWlorMnpwRUxMbnpTVng5em5IcEJjQUJBL1VHSHBYdyszd3NuT0JFdDVSNnFwNUpZYUVZMWJYa2hvVEw0SFNnQkxoNUV1dmlrZ2ZiZDRzcWhLTFdLUU5Id1NEcFdwUmM4MDd1cHpybHpQV1R2QXhmS2hyczFQNytFZjF6bjZrMHRZZ1ZCSmR1L0prckY3Y0dRbmpJTnlkdm96RExNQmVaS1FUVVpQZ2xyOFhMNWJ0TVFaT1ppdmg1QWtzNGRlbDBrUWZnaHNsTm52RU4yTTMyQ3YvZzFpTmhwaU13Y2UzZ0NRVXdTZU5BL2hFQUpraldOMFdJempIZFpCa251Q21oT2RQTmc9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfNDlkZjhhZjctY2Q3ZC00MWQ4LWJjYTMtODM1MDVmNWU4NzAxIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmUwYWIwZGJiLTVlMjEtNGQ0NS04Yzc0LTQxMDBlOTAzZTk3OTwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDEzOjE1OjU2LjYxOVoiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMzowMDo1Ni42MTlaIiBTZXNzaW9uSW5kZXg9ImUwMTVjMGJmLTRiZmMtNDI0Ny04NTQ0LTk3ODA2YTZmZGMzODozZDgzYjk3NmM4YWIwZjFiZGJhYWE4MjEzYWFhMDM3YzUyZWVmMGE4MjBiMDk3YzlmOGY3YWM5MDhlNGU3MGJiIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkpvemVmPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9Pc29iYS9QcmllenZpc2tvIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+TsOha292a2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MTIuMDguMTk5NDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj44MEUzNkMzRC1DN0YxLTQzNzAtOUQ4Ni1DRDFGM0IxRERDQzQ8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MDgxMDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcwODEwPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930570810&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;birthNumber&quot;:&quot;940812/8895&quot;,&quot;genderCode&quot;:&quot;1&quot;,&quot;dateOfBirth&quot;:&quot;1994-08-12&quot;,&quot;name&quot;:&quot;Jozef&quot;,&quot;surname&quot;:&quot;Nákovka&quot;,&quot;subjectGuid&quot;:&quot;c1d4781e-f2fd-4eb5-98c3-1734a7015b21&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;identifikatorVIAM&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910016010&quot;,&quot;vybrataSaszNazov&quot;:&quot;Základná škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910016010&quot;,&quot;nazov&quot;:&quot;Základná škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-15&quot;}]}};
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
      <webElementGuid>aace19be-3f9e-471f-a02d-ba89983e49ff</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]</value>
      <webElementGuid>3c6f71b0-d208-41f1-89f0-2d38fba740e2</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>caa043b3-424e-4281-a6f0-4e2a4edafc7f</webElementGuid>
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
                            
                        
                    
                            
                                
                                    
                                        JN
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const pageTextsDieta = {
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
        },
        krstneMeno: {
            label: &quot;Krstné meno&quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        priezvisko: {
            label: &quot;Priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        rodnePriezvisko: {
            label: &quot;Rodné priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        datumNarodenia: {
            label: &quot;Dátum narodenia&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        pohlavieRadio: {
            label: &quot;Pohlavie&quot;,
            options: [
                {
                    label: &quot;Muž&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Žena&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;
        },
        miestoNarodenia: {
            label: &quot;Miesto narodenia&quot;,
            required: true,
            regexError: &quot;Zadajte miesto narodenia.&quot;,
        },
        narodnost: {
            label: &quot;Národnosť&quot;,
            required: true
        },
        statnaPrislusnost: {
            label: &quot;Štátna príslušnosť&quot;,
            selectError: &quot;Vyhľadajte štát po zadaní prvých 3 znakov a vyberte štát zo zoznamu zobrazených štátov!&quot;,
            required: true
        },
        materinskyJazyk: {
            label: &quot;Materinský jazyk&quot;,
            required: true
        },
        inyMaterinskyJazyk: {
            label: &quot;Iný materinský jazyk&quot;,
            required: false
        },
        trvalyPobytRadio: {
            label: &quot;Adresa trvalého pobytu dieťaťa&quot;,
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;#####&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná adresa trvalého pobytu&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        zvycajnaAdresaradio: {
            label: &quot;Adresa miesta, kde dieťa zvyčajne žije&quot;,
            hint: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.&quot;,
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;Zhodná s adresou trvalého pobytu dieťaťa&quot; , &quot;'&quot; , &quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná adresa&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90346.01
    

    
        Podrobnosti o dieťati
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
            
                Všeobecné informácie
                
                    

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Krstné meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          falseClear date    

                
                
                    

    
    
        Pohlavie
        (nepovinné)
    
    
    
    
                        
                        Muž 
                         
                        
                        Žena 
                         
    

                
            

            
                Štátna príslušnosť a jazyk
                
                    

    
        Miesto narodenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Národnosť
        (nepovinné)
    
    
    
        
        slovenská
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
abchádzskyafanoromskýafarskýafgánskyajmarskýalbánskyamharskýanglickýarabskýarménskyasamskýazerbajdžanskýBarcianskybaskickýbaškirskýbengálskybhútanskýbieloruskýbiharskýbirmskýbislamskýbretónskybulharskýčeskýceylonský /singalezština/chorvátskyčínskydánskyeskimáckyesperantoestónskyfaerskýfidžijskýfínskyflámskyfrancúzskyfrízskygréckygrónskygruzínskyguaranígudžarátskyhaličskýhaustskýhebrejskýhindustánskyholandskýindonézskyinterliguaskýinupiakskýinýiný (mŕtvy jazyk)írskyislandskýjaponskýjavanskýjidišskýjuhoafrická holandštinakambodžskýkanadskýkašmírskykatalánskykazachskýkinjarwandskýkirgizskýkirundikórejskýkorzickýkurdskýlaoskýlatinskýlingalskýlitovskýlotyšskýmacedónskymadagaskarskýmaďarskýmalabarskýmalajzskýmaltézskyMaori /jazyk novozélandskej rasy/maráthskýmohamedánska hindustuuštinamongolskýmultanský /moldavský/nautskýnemeckýnepálskynesleduje sanórskyocitanskýorjanskýpandžábskyperzskýpoľskýportugalskýrhaetorománskyrómskyrumunskýrusínskyruskýsamojskýsangskýsanskritskýsindhinskýsiswatskýškótskyslovenskýslovinskýsomálskyšonskýsothošpanielskysrbochorvátskysrbskýstwanskýsuahelisudánskyšvédskytadžickýtagalskýtalianskytamilskýťanský (ťvi)tatarskýtelugskýthajskýtibetskýtigrinskýtongskýtureckýturkménskyudmurský (voťačský)ukrajinskýuzbeckývietnamskýwaleskýwelskýwolofskýxhosayorubský (yoruba)židovskýzulu /kafr/

                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
český

                

            

            
                Trvalý pobyt
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaTPControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaTPKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPObec: {
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
                required: false,
                    attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPUlicaReq: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPSupisneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaTPOrientacneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaTPPSC: {
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
               {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaTPAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
                    

    
    
        Adresa miesta, kde dieťa zvyčajne žije
        (nepovinné)
    
    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaZAControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZAKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAObec: {
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
                required: false,
                    attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAUlicaReq: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZASupisneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaZAOrientacneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaZAPSC: {
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
               {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaZAAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
            

            
                Predprimárne vzdelávanie
                
                    

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
            

        

    
        Zrušiť
        
            Uložiť a späť na prihlášku
        
    



    
    
        
            
                
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
        window.userData = {&quot;nameId&quot;:&quot;e0ab0dbb-5e21-4d45-8c74-4100e903e979&quot;,&quot;tokenDescriptor&quot;:&quot;e015c0bf-4bfc-4247-8544-97806a6fdc38:3d83b976c8ab0f1bdbaaa8213aaa037c52eef0a820b097c9f8f7ac908e4e70bb&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930570810&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgSUQ9Il84M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI184M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT43Wi9sckI5ZG9sUWNoQUZzdi9BVXc4ZTNkUzFzYzlSc250cmlSSS9lWkg4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5ESmtDQmVFblVRc3NTVkdoQXFobzEwMzgwSzU1cllLR2ZSZloyZmhUYnFFQVVZeUpwd3VJNHlHVG5FR05QbTkyWkxkOXBBTmgrZFBSSll4V2xTWlorMnpwRUxMbnpTVng5em5IcEJjQUJBL1VHSHBYdyszd3NuT0JFdDVSNnFwNUpZYUVZMWJYa2hvVEw0SFNnQkxoNUV1dmlrZ2ZiZDRzcWhLTFdLUU5Id1NEcFdwUmM4MDd1cHpybHpQV1R2QXhmS2hyczFQNytFZjF6bjZrMHRZZ1ZCSmR1L0prckY3Y0dRbmpJTnlkdm96RExNQmVaS1FUVVpQZ2xyOFhMNWJ0TVFaT1ppdmg1QWtzNGRlbDBrUWZnaHNsTm52RU4yTTMyQ3YvZzFpTmhwaU13Y2UzZ0NRVXdTZU5BL2hFQUpraldOMFdJempIZFpCa251Q21oT2RQTmc9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfNDlkZjhhZjctY2Q3ZC00MWQ4LWJjYTMtODM1MDVmNWU4NzAxIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmUwYWIwZGJiLTVlMjEtNGQ0NS04Yzc0LTQxMDBlOTAzZTk3OTwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDEzOjE1OjU2LjYxOVoiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMzowMDo1Ni42MTlaIiBTZXNzaW9uSW5kZXg9ImUwMTVjMGJmLTRiZmMtNDI0Ny04NTQ0LTk3ODA2YTZmZGMzODozZDgzYjk3NmM4YWIwZjFiZGJhYWE4MjEzYWFhMDM3YzUyZWVmMGE4MjBiMDk3YzlmOGY3YWM5MDhlNGU3MGJiIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkpvemVmPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9Pc29iYS9QcmllenZpc2tvIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+TsOha292a2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MTIuMDguMTk5NDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj44MEUzNkMzRC1DN0YxLTQzNzAtOUQ4Ni1DRDFGM0IxRERDQzQ8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MDgxMDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcwODEwPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930570810&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;birthNumber&quot;:&quot;940812/8895&quot;,&quot;genderCode&quot;:&quot;1&quot;,&quot;dateOfBirth&quot;:&quot;1994-08-12&quot;,&quot;name&quot;:&quot;Jozef&quot;,&quot;surname&quot;:&quot;Nákovka&quot;,&quot;subjectGuid&quot;:&quot;c1d4781e-f2fd-4eb5-98c3-1734a7015b21&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;identifikatorVIAM&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910016010&quot;,&quot;vybrataSaszNazov&quot;:&quot;Základná škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910016010&quot;,&quot;nazov&quot;:&quot;Základná škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-15&quot;}]}};
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
                            
                        
                    
                            
                                
                                    
                                        JN
                                    
                                
                                
                                    
                                        
                                            Môj profil
                                        
                                    
                                    
                                        
                                            Oznámenia
                                        
                                    
                                    
                                        Odhlásiť
                                    
                                
                            
                
            

            
                Menu
                
                    
                                
                                    
                                        Prihlášky a rozhodnutia
                                    
                                
                                    
                                    
                                        Správa používateľov
                                    
                                
                                
                                    
                                        Pomoc a podpora
                                    
                                
                    
                
            
        
    

    

    const pageTextsDieta = {
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
        },
        krstneMeno: {
            label: &quot;Krstné meno&quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        priezvisko: {
            label: &quot;Priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        rodnePriezvisko: {
            label: &quot;Rodné priezvisko&quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        datumNarodenia: {
            label: &quot;Dátum narodenia&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        pohlavieRadio: {
            label: &quot;Pohlavie&quot;,
            options: [
                {
                    label: &quot;Muž&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Žena&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;
        },
        miestoNarodenia: {
            label: &quot;Miesto narodenia&quot;,
            required: true,
            regexError: &quot;Zadajte miesto narodenia.&quot;,
        },
        narodnost: {
            label: &quot;Národnosť&quot;,
            required: true
        },
        statnaPrislusnost: {
            label: &quot;Štátna príslušnosť&quot;,
            selectError: &quot;Vyhľadajte štát po zadaní prvých 3 znakov a vyberte štát zo zoznamu zobrazených štátov!&quot;,
            required: true
        },
        materinskyJazyk: {
            label: &quot;Materinský jazyk&quot;,
            required: true
        },
        inyMaterinskyJazyk: {
            label: &quot;Iný materinský jazyk&quot;,
            required: false
        },
        trvalyPobytRadio: {
            label: &quot;Adresa trvalého pobytu dieťaťa&quot;,
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;#####&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná adresa trvalého pobytu&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        zvycajnaAdresaradio: {
            label: &quot;Adresa miesta, kde dieťa zvyčajne žije&quot;,
            hint: &quot;Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.&quot;,
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;Zhodná s adresou trvalého pobytu dieťaťa&quot; , &quot;'&quot; , &quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná adresa&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90346.01
    

    
        Podrobnosti o dieťati
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
            
                Všeobecné informácie
                
                    

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Krstné meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        Pick a date
        warning
        warning
        
            calendar_month
        
    
    «F Y»PoUtStŠtPiSoNe                                          falseClear date    

                
                
                    

    
    
        Pohlavie
        (nepovinné)
    
    
    
    
                        
                        Muž 
                         
                        
                        Žena 
                         
    

                
            

            
                Štátna príslušnosť a jazyk
                
                    

    
        Miesto narodenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Národnosť
        (nepovinné)
    
    
    
        
        slovenská
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
abchádzskyafanoromskýafarskýafgánskyajmarskýalbánskyamharskýanglickýarabskýarménskyasamskýazerbajdžanskýBarcianskybaskickýbaškirskýbengálskybhútanskýbieloruskýbiharskýbirmskýbislamskýbretónskybulharskýčeskýceylonský /singalezština/chorvátskyčínskydánskyeskimáckyesperantoestónskyfaerskýfidžijskýfínskyflámskyfrancúzskyfrízskygréckygrónskygruzínskyguaranígudžarátskyhaličskýhaustskýhebrejskýhindustánskyholandskýindonézskyinterliguaskýinupiakskýinýiný (mŕtvy jazyk)írskyislandskýjaponskýjavanskýjidišskýjuhoafrická holandštinakambodžskýkanadskýkašmírskykatalánskykazachskýkinjarwandskýkirgizskýkirundikórejskýkorzickýkurdskýlaoskýlatinskýlingalskýlitovskýlotyšskýmacedónskymadagaskarskýmaďarskýmalabarskýmalajzskýmaltézskyMaori /jazyk novozélandskej rasy/maráthskýmohamedánska hindustuuštinamongolskýmultanský /moldavský/nautskýnemeckýnepálskynesleduje sanórskyocitanskýorjanskýpandžábskyperzskýpoľskýportugalskýrhaetorománskyrómskyrumunskýrusínskyruskýsamojskýsangskýsanskritskýsindhinskýsiswatskýškótskyslovenskýslovinskýsomálskyšonskýsothošpanielskysrbochorvátskysrbskýstwanskýsuahelisudánskyšvédskytadžickýtagalskýtalianskytamilskýťanský (ťvi)tatarskýtelugskýthajskýtibetskýtigrinskýtongskýtureckýturkménskyudmurský (voťačský)ukrajinskýuzbeckývietnamskýwaleskýwelskýwolofskýxhosayorubský (yoruba)židovskýzulu /kafr/

                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        český
        warning
        
            keyboard_arrow_down
        
    
český

                

            

            
                Trvalý pobyt
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaTPControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaTPKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPObec: {
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
                required: false,
                    attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPUlicaReq: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaTPSupisneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaTPOrientacneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaTPPSC: {
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
               {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaTPAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
                    

    
    
        Adresa miesta, kde dieťa zvyčajne žije
        (nepovinné)
    
    Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu.
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Ilava (Ilava)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Technická
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        Adresa
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaZAControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZAKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAObec: {
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
                required: false,
                    attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZAUlicaReq: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;
                }
            ]
        },
        adresaZASupisneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte súpisné číslo.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{1,10}$/,
                    message: &quot;Zadajte súpisné číslo.&quot;
                }
            ]
        },
        adresaZAOrientacneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte orientačné číslo.&quot;,
            required: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,20}$/,
                    message: &quot;Zadajte orientačné číslo.&quot;
                }
            ]
        },
        adresaZAPSC: {
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;
            },
            validators: [
               {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;
                }
            ]
        },
        adresaZAAdresa: {
            label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,
            required: true,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                    message: &quot;Zadajte zahraničnú adresu.&quot;
                }
            ]
        }
    };


                
            

            
                Predprimárne vzdelávanie
                
                    

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
            

        

    
        Zrušiť
        
            Uložiť a späť na prihlášku
        
    



    
    
        
            
                
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
        window.userData = {&quot;nameId&quot;:&quot;e0ab0dbb-5e21-4d45-8c74-4100e903e979&quot;,&quot;tokenDescriptor&quot;:&quot;e015c0bf-4bfc-4247-8544-97806a6fdc38:3d83b976c8ab0f1bdbaaa8213aaa037c52eef0a820b097c9f8f7ac908e4e70bb&quot;,&quot;sessionNotBeforeUtc&quot;:&quot;2025-05-29&quot;,&quot;sessionNotOnOrAfterUtc&quot;:&quot;2025-05-30&quot;,&quot;authType&quot;:&quot;W&quot;,&quot;accountName&quot;:&quot;dc.iedu.sk\\930570810&quot;,&quot;actorId&quot;:null,&quot;subjectId&quot;:null,&quot;preferredLanguage&quot;:&quot;SK&quot;,&quot;additionalInfo&quot;:null,&quot;permissions&quot;:[&quot;uri://ESRaVS/PES/PrivatnaZonaUser&quot;,&quot;uri://IAM/CORE/Account/MyPwdChange&quot;,&quot;uri://RIS/WEB/OsobaPrihlasky&quot;,&quot;uri://RIS/WEB/PrihlaskyDieta&quot;,&quot;uri://RIS/WEB/VyhladanieSaSZ&quot;,&quot;uri://RIS/WEB/WEB_Ciselniky&quot;,&quot;uri://RIS/Zamestnanec&quot;],&quot;samlResponse&quot;:{&quot;relayState&quot;:&quot;/&quot;,&quot;samlMessageFromIamXml&quot;:&quot;PHNhbWwycDpSZXNwb25zZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4bWxuczpzYW1sMnA9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpwcm90b2NvbCIgeG1sbnM6ZHM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvMDkveG1sZHNpZyMiIHhtbG5zOnNhbWwyPSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXNzZXJ0aW9uIiB4bWxuczp4cz0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEiIHhtbG5zOnhlbmM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMDQveG1sZW5jIyIgeG1sbnM6bWQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDptZXRhZGF0YSIgVmVyc2lvbj0iMi4wIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgSUQ9Il84M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgRGVzdGluYXRpb249Imh0dHBzOi8vdGVzdC1lcHJpaGxhc2t5LmllZHUuc2svYXNzZXJ0aW9uY29uc3VtZXJzZXJ2aWNlIj48c2FtbDI6SXNzdWVyIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOm5hbWVpZC1mb3JtYXQ6ZW50aXR5Ij5odHRwczovL3RpYW0uaWVkdS5zay9zYW1sMi9tZXRhPC9zYW1sMjpJc3N1ZXI+PGRzOlNpZ25hdHVyZT48ZHM6U2lnbmVkSW5mbz48ZHM6Q2Fub25pY2FsaXphdGlvbk1ldGhvZCBBbGdvcml0aG09Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvMTAveG1sLWV4Yy1jMTRuIyIgLz48ZHM6U2lnbmF0dXJlTWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxkc2lnLW1vcmUjcnNhLXNoYTI1NiIgLz48ZHM6UmVmZXJlbmNlIFVSST0iI184M2ZmMjBkYy04ODc0LTQ0NDUtOTM3Yi03OGIyMTNiM2QxNjciPjxkczpUcmFuc2Zvcm1zPjxkczpUcmFuc2Zvcm0gQWxnb3JpdGhtPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwLzA5L3htbGRzaWcjZW52ZWxvcGVkLXNpZ25hdHVyZSIgLz48ZHM6VHJhbnNmb3JtIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8xMC94bWwtZXhjLWMxNG4jIiAvPjwvZHM6VHJhbnNmb3Jtcz48ZHM6RGlnZXN0TWV0aG9kIEFsZ29yaXRobT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS8wNC94bWxlbmMjc2hhMjU2IiAvPjxkczpEaWdlc3RWYWx1ZT43Wi9sckI5ZG9sUWNoQUZzdi9BVXc4ZTNkUzFzYzlSc250cmlSSS9lWkg4PTwvZHM6RGlnZXN0VmFsdWU+PC9kczpSZWZlcmVuY2U+PC9kczpTaWduZWRJbmZvPjxkczpTaWduYXR1cmVWYWx1ZT5ESmtDQmVFblVRc3NTVkdoQXFobzEwMzgwSzU1cllLR2ZSZloyZmhUYnFFQVVZeUpwd3VJNHlHVG5FR05QbTkyWkxkOXBBTmgrZFBSSll4V2xTWlorMnpwRUxMbnpTVng5em5IcEJjQUJBL1VHSHBYdyszd3NuT0JFdDVSNnFwNUpZYUVZMWJYa2hvVEw0SFNnQkxoNUV1dmlrZ2ZiZDRzcWhLTFdLUU5Id1NEcFdwUmM4MDd1cHpybHpQV1R2QXhmS2hyczFQNytFZjF6bjZrMHRZZ1ZCSmR1L0prckY3Y0dRbmpJTnlkdm96RExNQmVaS1FUVVpQZ2xyOFhMNWJ0TVFaT1ppdmg1QWtzNGRlbDBrUWZnaHNsTm52RU4yTTMyQ3YvZzFpTmhwaU13Y2UzZ0NRVXdTZU5BL2hFQUpraldOMFdJempIZFpCa251Q21oT2RQTmc9PTwvZHM6U2lnbmF0dXJlVmFsdWU+PGRzOktleUluZm8+PGRzOlg1MDlEYXRhPjxkczpYNTA5Q2VydGlmaWNhdGU+TUlJQ3R6Q0NBWitnQXdJQkFnSUpBTkdCdkNSWGZoTjFNQTBHQ1NxR1NJYjNEUUVCQ3dVQU1Cc3hHVEFYQmdOVkJBTU1FR2xrY0M1MGFXRnRMbWxsWkhVdWMyc3dIaGNOTWpRd016RXlNVEUwT0RFMldoY05NelF3TXpFeU1URTBPREUyV2pBYk1Sa3dGd1lEVlFRRERCQnBaSEF1ZEdsaGJTNXBaV1IxTG5Ock1JSUJJakFOQmdrcWhraUc5dzBCQVFFRkFBT0NBUThBTUlJQkNnS0NBUUVBdVdwamdJN1RrYzVVMHY2K3FjWERzQjF2ZHdpZlBXanJ4dVozK2YrT3ZaNldVQkFBNFhDaXp5dDAydEMvd0lRUkdOZU50SmwraHlpQWZBYitkNGZsQnNsTTc0T21yQTJLZVlUZElpRnZqRXBpRlVSV3o4ZHVKWjdqWkZ4blJ0Zlp2OW1tQWF2SFpJK0F2cXJBWG5QcDhrVUxOWWM2enVNV3NhUUIxcGdUU0wzQWV6L3hLTERPNnFQVW9nZEt0VDFnWnlxM3pmcDdBL1Axa1pqMmZOWHlnYlNFZW0wUmFEcnNHcSs3eXEvbHlNNk43WStldTFhbEVYb3lpSzlrRnRLYlN6Rzc3TC9hSHd3Ty9KMHE1RmdtTWJkYkprc3MrY0FBS3FKMXcvdm4rUU5iek9MOWZmNG0xZ2E2TFRUMmdGZ0lJTE5hcjhDMTdoS3BGdUtqdjJwYW53SURBUUFCTUEwR0NTcUdTSWIzRFFFQkN3VUFBNElCQVFBM29obWFJNG9BK1RFQ2x2djMyN093NmgwdXFVWUkwT0JXTUlydE9JeG9ZYWI1eHhuSGdLUkViMFVhNytUQVgwMnhoa0h1dW8wa1A4RDZ1T0xuN2JxRFVSdmtpNzlteGhtRlFReDdxb3gvT2ZndWk4Nk1hb3lQOHB0S21ESmszeGxrRy9nSDJ5b1VUVW51MDZIbjJmUmVBVy9icHI1TVVvVG5LMVlkSnZlUVJWRFRPOXVJRVJTM3RiOTdDZjNBNW9TanpCb3lML05mRERnd1h0a0hGeHozUTJDcWIzZ2VKUHNHMGw2L1M0WUNZaG9veXdOU2ZpSnNxenN1Z21EQ2x2STRHcWlINFN0UHhXQytCUFFEcG0vTE00bHRkWjhzeTFWbFFRc1ZLL2V5Rnh0azl4UklRL2ppUU41aEpTVVBwK3dYTm9JR1ZvVG9Cbnk4T3VEdmRTazc8L2RzOlg1MDlDZXJ0aWZpY2F0ZT48L2RzOlg1MDlEYXRhPjwvZHM6S2V5SW5mbz48L2RzOlNpZ25hdHVyZT48c2FtbDJwOlN0YXR1cz48c2FtbDJwOlN0YXR1c0NvZGUgVmFsdWU9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpzdGF0dXM6U3VjY2VzcyIgLz48c2FtbDJwOlN0YXR1c01lc3NhZ2U+dXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOnN0YXR1czpTdWNjZXNzPC9zYW1sMnA6U3RhdHVzTWVzc2FnZT48L3NhbWwycDpTdGF0dXM+PHNhbWwyOkFzc2VydGlvbiBWZXJzaW9uPSIyLjAiIElEPSJfNDlkZjhhZjctY2Q3ZC00MWQ4LWJjYTMtODM1MDVmNWU4NzAxIiBJc3N1ZUluc3RhbnQ9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiI+PHNhbWwyOklzc3VlciBGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDpuYW1laWQtZm9ybWF0OmVudGl0eSI+aHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YTwvc2FtbDI6SXNzdWVyPjxzYW1sMjpTdWJqZWN0PjxzYW1sMjpOYW1lSUQgTmFtZVF1YWxpZmllcj0iaHR0cHM6Ly90aWFtLmllZHUuc2svc2FtbDIvbWV0YSIgU1BOYW1lUXVhbGlmaWVyPSJodHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGEiIEZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6MS4xOm5hbWVpZC1mb3JtYXQ6dW5zcGVjaWZpZWQiPmUwYWIwZGJiLTVlMjEtNGQ0NS04Yzc0LTQxMDBlOTAzZTk3OTwvc2FtbDI6TmFtZUlEPjxzYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uIE1ldGhvZD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmNtOmJlYXJlciI+PHNhbWwyOlN1YmplY3RDb25maXJtYXRpb25EYXRhIEluUmVzcG9uc2VUbz0iX2M4OWIwNTVmLTMwZTItNDk1ZC05YjQ4LTIwNzEyM2NlZjU1YyIgTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiIFJlY2lwaWVudD0iaHR0cHM6Ly90ZXN0LWVwcmlobGFza3kuaWVkdS5zay9hc3NlcnRpb25jb25zdW1lcnNlcnZpY2UiIC8+PC9zYW1sMjpTdWJqZWN0Q29uZmlybWF0aW9uPjwvc2FtbDI6U3ViamVjdD48c2FtbDI6Q29uZGl0aW9ucyBOb3RCZWZvcmU9IjIwMjUtMDUtMjlUMTM6MDA6NTYuNjE5WiIgTm90T25PckFmdGVyPSIyMDI1LTA1LTI5VDEzOjE1OjU2LjYxOVoiPjxzYW1sMjpBdWRpZW5jZVJlc3RyaWN0aW9uPjxzYW1sMjpBdWRpZW5jZT5odHRwczovL3Rlc3QtZXByaWhsYXNreS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1ZGllbmNlPjwvc2FtbDI6QXVkaWVuY2VSZXN0cmljdGlvbj48L3NhbWwyOkNvbmRpdGlvbnM+PHNhbWwyOkF1dGhuU3RhdGVtZW50IEF1dGhuSW5zdGFudD0iMjAyNS0wNS0yOVQxMzowMDo1Ni42MTlaIiBTZXNzaW9uSW5kZXg9ImUwMTVjMGJmLTRiZmMtNDI0Ny04NTQ0LTk3ODA2YTZmZGMzODozZDgzYjk3NmM4YWIwZjFiZGJhYWE4MjEzYWFhMDM3YzUyZWVmMGE4MjBiMDk3YzlmOGY3YWM5MDhlNGU3MGJiIiBTZXNzaW9uTm90T25PckFmdGVyPSIyMDI1LTA1LTMwVDAxOjAwOjU2LjQzN1oiPjxzYW1sMjpTdWJqZWN0TG9jYWxpdHkgLz48c2FtbDI6QXV0aG5Db250ZXh0PjxzYW1sMjpBdXRobkNvbnRleHRDbGFzc1JlZj51cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YWM6Y2xhc3NlczpQYXNzd29yZDwvc2FtbDI6QXV0aG5Db250ZXh0Q2xhc3NSZWY+PHNhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pmh0dHBzOi8vdGlhbS5pZWR1LnNrL3NhbWwyL21ldGE8L3NhbWwyOkF1dGhlbnRpY2F0aW5nQXV0aG9yaXR5Pjwvc2FtbDI6QXV0aG5Db250ZXh0Pjwvc2FtbDI6QXV0aG5TdGF0ZW1lbnQ+PHNhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9VY2V0L1R5cEF1dGVudGlmaWthY2llIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+Vzwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT3NvYmEvTWVubyIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPkpvemVmPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9Pc29iYS9QcmllenZpc2tvIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+TsOha292a2E8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0RhdHVtTmFyb2RlbmlhIiBOYW1lRm9ybWF0PSJ1cm46b2FzaXM6bmFtZXM6dGM6U0FNTDoyLjA6YXR0cm5hbWUtZm9ybWF0OmJhc2ljIj48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+MTIuMDguMTk5NDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vT1NPQkFfSUQiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj44MEUzNkMzRC1DN0YxLTQzNzAtOUQ4Ni1DRDFGM0IxRERDQzQ8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0iRW1haWwiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5iYXJjaWtAZGl0ZWMuc2s8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjwvc2FtbDI6QXR0cmlidXRlPjxzYW1sMjpBdHRyaWJ1dGUgTmFtZT0idXJpOi8vSUFNL09zb2JhL0tvbnRha3QiIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj5Lb2QoRU1BSUwpLyhiYXJjaWtAZGl0ZWMuc2spPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9InVyaTovL0lBTS9JRCIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPjkzMDU3MDgxMDwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PC9zYW1sMjpBdHRyaWJ1dGU+PHNhbWwyOkF0dHJpYnV0ZSBOYW1lPSJ1cmk6Ly9JQU0vVWNldC9OYXpvdiIgTmFtZUZvcm1hdD0idXJuOm9hc2lzOm5hbWVzOnRjOlNBTUw6Mi4wOmF0dHJuYW1lLWZvcm1hdDpiYXNpYyI+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPmRjLmllZHUuc2tcOTMwNTcwODEwPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48c2FtbDI6QXR0cmlidXRlIE5hbWU9IlBlcm1pc3Npb24iIE5hbWVGb3JtYXQ9InVybjpvYXNpczpuYW1lczp0YzpTQU1MOjIuMDphdHRybmFtZS1mb3JtYXQ6YmFzaWMiPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9FU1JhVlMvUEVTL1ByaXZhdG5hWm9uYVVzZXI8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9JQU0vQ09SRS9BY2NvdW50L015UHdkQ2hhbmdlPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9Pc29iYVByaWhsYXNreTwvc2FtbDI6QXR0cmlidXRlVmFsdWU+PHNhbWwyOkF0dHJpYnV0ZVZhbHVlIHhtbG5zOnhzaT0iaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEtaW5zdGFuY2UiIHhzaTp0eXBlPSJ4czpzdHJpbmciPnVyaTovL1JJUy9XRUIvUHJpaGxhc2t5RGlldGE8L3NhbWwyOkF0dHJpYnV0ZVZhbHVlPjxzYW1sMjpBdHRyaWJ1dGVWYWx1ZSB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiB4c2k6dHlwZT0ieHM6c3RyaW5nIj51cmk6Ly9SSVMvV0VCL1Z5aGxhZGFuaWVTYVNaPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1dFQi9XRUJfQ2lzZWxuaWt5PC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48c2FtbDI6QXR0cmlidXRlVmFsdWUgeG1sbnM6eHNpPSJodHRwOi8vd3d3LnczLm9yZy8yMDAxL1hNTFNjaGVtYS1pbnN0YW5jZSIgeHNpOnR5cGU9InhzOnN0cmluZyI+dXJpOi8vUklTL1phbWVzdG5hbmVjPC9zYW1sMjpBdHRyaWJ1dGVWYWx1ZT48L3NhbWwyOkF0dHJpYnV0ZT48L3NhbWwyOkF0dHJpYnV0ZVN0YXRlbWVudD48L3NhbWwyOkFzc2VydGlvbj48L3NhbWwycDpSZXNwb25zZT4=&quot;,&quot;samlMessageFromUpvsXml&quot;:null,&quot;statusCode&quot;:&quot;urn:oasis:names:tc:SAML:2.0:status:Success&quot;,&quot;secondLevelStatusCode&quot;:null},&quot;person&quot;:{&quot;identifier&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;isActingOnOwnBehalf&quot;:1,&quot;ifo&quot;:null,&quot;eduId&quot;:&quot;930570810&quot;,&quot;email&quot;:&quot;barcik@ditec.sk&quot;,&quot;phone&quot;:null,&quot;loggedInPersonGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;birthNumber&quot;:&quot;940812/8895&quot;,&quot;genderCode&quot;:&quot;1&quot;,&quot;dateOfBirth&quot;:&quot;1994-08-12&quot;,&quot;name&quot;:&quot;Jozef&quot;,&quot;surname&quot;:&quot;Nákovka&quot;,&quot;subjectGuid&quot;:&quot;c1d4781e-f2fd-4eb5-98c3-1734a7015b21&quot;,&quot;ico&quot;:null,&quot;title&quot;:null,&quot;upvsIdentityId&quot;:null,&quot;subjectEmail&quot;:null,&quot;mailboxNumber&quot;:null,&quot;mailboxUri&quot;:null,&quot;mailboxStatus&quot;:null,&quot;representationType&quot;:null,&quot;typeOfOrganisation&quot;:null,&quot;prihlasenaOsobaGuid&quot;:&quot;5fe53d49-dd43-4180-9b17-1114d9636b89&quot;,&quot;identifikatorVIAM&quot;:&quot;80E36C3D-C7F1-4370-9D86-CD1F3B1DDCC4&quot;,&quot;identifikatorVIAMUPVS&quot;:null,&quot;citizenOfSlovakia&quot;:null,&quot;vybrataSaszEduid&quot;:&quot;910016010&quot;,&quot;vybrataSaszNazov&quot;:&quot;Základná škola&quot;,&quot;vybrataSaszTypPouzivatelaKod&quot;:&quot;1&quot;,&quot;zoznamSasz&quot;:[{&quot;eduId&quot;:&quot;910016010&quot;,&quot;nazov&quot;:&quot;Základná škola&quot;,&quot;typPouzivatelaKod&quot;:&quot;1&quot;,&quot;naposledyPouzita&quot;:true,&quot;platnostOd&quot;:&quot;2025-05-15&quot;}]}};
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
      <webElementGuid>90126b97-b614-474d-b7ab-a8ae3130ffb7</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
