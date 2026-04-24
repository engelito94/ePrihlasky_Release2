<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Odhlsi_govuk-wizard-steps</name>
   <tag></tag>
   <elementGuidId>cb1ac314-081a-40ad-8ff4-fc8f127c9d8e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='ziadost']/div/div/div/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.govuk-wizard-steps</value>
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
      <value>div</value>
      <webElementGuid>d8e4410c-9e5c-4b90-af8b-9691f7ef1da1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-wizard-steps</value>
      <webElementGuid>97259aa5-5bee-4371-b5f1-aa0c6e3f434b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    

    
                        
                            Správa prihlášok
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    
                    
                        
                            
                                Krok3/7
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje dieťaťa
                        
                    
                        
                            2.
                            Doplňujúce údaje o dieťati
                        
                    
                        
                            3.
                            Vybrať školy
                        
                    
                        
                            4.
                            Zákonný zástupca dieťaťa
                        
                    
                        
                            5.
                            Pridať prílohy
                        
                    
                        
                            6.
                            Ostatné údaje
                        
                    
                        
                            7.
                            Súhrnný prehľad
                        
                    
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: 'Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne',
        prosimCakajte: 'Prosím, čakajte',
        systemPracuje: 'Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.',
        '8020': &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        '8021': &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        'dietaNemaInyMaterinsky': &quot;Dieťa nemá iný materinský jazyk&quot;,
         inaAdresaTP: &quot;Iná adresa trvalého pobytu&quot;,
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má {osoba} rodné číslo?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: '1'
                },
                {
                    label: &quot;Nie&quot;,
                    value: '2'
                }
            ],
            required: true,
            direction: 'row',
            povinneError: &quot;Uveďte rodné číslo dieťaťa. Ak dieťa rodné číslo nemá, zvoľte možnosť \&quot;Nie\&quot;.&quot;,
        },
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
        },
        krstneMeno: {
            label: &quot;Meno&quot;,
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
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
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
            label: &quot;Adresa trvalého pobytu {osoba}&quot;,
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
            ],
        },
        zvycajnaAdresaradio: {
            label: &quot;Vyberte adresu&quot;,
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
        plnolety: {
            label: &quot;Uchádzač je plnoletý&quot;,
            element: 'plnolety',
            ano: &quot;Áno&quot;,
            nie: &quot;Nie&quot;,
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    Osobné údaje dieťaťa

    
        
            Pridať dieťa podľa rodného čísla.
        
        Polia označené hviezdičkou sú povinné
        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            
                        
                        Nie 
                         
    

            
            
        
    
    
        

            
                Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.
            

            Polia označené hviezdičkou sú povinné

            
                
                    
                        Osobné údaje dieťaťa
                    
                
                
                    
                        Meno
                        Tibor
                    
                    
                        Priezvisko
                        Lesko
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        200112/6853
                    
                    
                        Dátum narodenia
                        12.01.2020
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Slovensko
                    
                    
                        Národnosť
                        slovenská
                    
                    
                        Štátna príslušnosť
                        Slovenská republika
                    
                    
                        Materinský jazyk
                        slovenský
                    
                    
                        Iný materinský jazyk
                        -
                    
                    
                        Uchádzač je plnoletý
                        -
                    
                    
                        Adresa trvalého pobytu
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Predprimárne vzdelávanie
                        -
                    
                
            

            
                
                    
                        Osobné údaje dieťaťa
                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window['datumNarodeniaControlSettings'] = {
        datumNarodeniaDen: {
            label: 'Deň',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'DD',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumNarodeniaMesiac: {
            label: 'Mesiac',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'MM',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumNarodeniaRok: {
            label: 'Rok',
            showRequiredOrOptional: false,
            attributes: {
                minLength: '4',
                maxLength: '4',
                placeholder: 'YYYY',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
    
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
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        ##### 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    Slovenská republikaSlovenská republika – mimo číselník


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Predmier (Bytča)
        warning
        
            keyboard_arrow_down
        
    Predmier (Bytča)
Toto pole je povinné.

    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        Egrešská
        warning
        
            keyboard_arrow_down
        
    Egrešská


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window['adresaTPControlSettings'] = {
        adresaTPKrajina: {
            label: 'Krajina',
            hint: 'Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.',
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
            customOptionalText: '(ak existuje – potrebné pre doručovanie rozhodnutia)',
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window['adresaZAControlSettings'] = {
        adresaZAKrajina: {
            label: 'Krajina',
            hint: 'Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.',
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
            customOptionalText: '(ak existuje – potrebné pre doručovanie rozhodnutia)',
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


                        
                    
                    
                        

    
    
    
        
            
                
                    Uchádzač je plnoletý
                    (nepovinné)
                
                
            
            
        
    

                    
                

                
                    Predprimárne vzdelávanie
                    
                        

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

            
        
    


                    
                    
                        

    const controlSettingsDPD = {
        msCelodennaVychovaRadio : {
            label: &quot;Žiadam o prijatie dieťaťa na&quot;,
            options: [
                {
                    label: &quot;poldennú výchovu a vzdelávanie&quot;,
                    value: false
                },
                {
                    label: &quot;celodennú výchovu a vzdelávanie&quot;,
                    value: true
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        DPDSVVPRadio: {
            label: &quot;Zdravotné znevýhodnenie dieťaťa&quot;,
            hint: &quot;Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDDietaSNadanimRadio: {
            label: &quot;Dieťa s nadaním&quot;,
            hint: &quot;Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        zsDPDVychovaRadio : {
            label: &quot;Požadovaná výchova&quot;,
            options: [
                {
                    label: &quot;Etická&quot;,
                    value: '1'
                },
                {
                    label: &quot;Náboženská&quot;,
                    value: '2'
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiRadio : {
            label: &quot;Vyberte typ&quot;,
            options: [
                {
                    label: &quot;Rímskokatolícka&quot;,
                    value: 'Rímskokatolícka'
                },
                {
                    label: &quot;Evanjelická&quot;,
                    value: 'Evanjelická'
                },
                {
                    label: &quot;Pravoslávna&quot;,
                    value: 'Pravoslávna'
                },
                {
                    label: &quot;Reformovaná&quot;,
                    value: 'Reformovaná'
                },
                {
                    label: &quot;Gréckokatolícka&quot;,
                    value: 'Gréckokatolícka'
                },
                {
                    label: &quot;Iná. Zadajte typ:&quot;,
                    value: 'Iná'
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]
        },
        DPDPopisNadaniaText: {
            label: &quot;Popis nadania&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v školskej jedálni&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDSkolskyKlubRadio : {
            label: &quot;Záujem o Školský klub detí&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            hintZsMs: &quot;Môžete uviesť doplňujúce informácie, &lt;b>stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti&lt;/b>, ktoré môžu ovplyvniť vzdelávanie žiaka. &lt;br>Ak má dieťa &lt;b>súrodenca na škole&lt;/b>, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
            anyDate: true,
            validators: [],
            povinneError: &quot;Toto pole je povinné.&quot;
        },
        zmenenaPracovnaSchopnostRadio : {
            label: &quot;Má žiak zmenenú pracovnú schopnosť?&quot;,
            hint: &quot;Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte \&quot;Áno\&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        specialneVVPRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;So zdravotným znevýhodnením&quot;,
                    hint: &quot;Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.&quot;,
                    value: '1'
                },
                {
                    label: &quot;Zo sociálne znevýhodneného prostredia&quot;,
                    hint: &quot;Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.&quot;,
                    value: '2'
                },
                {
                    label: &quot;S nadaním&quot;,
                    hint: &quot;Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
                    value: '3'
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        mentalnePostihnutieRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;Mentálne postihnutie&quot;,
                    value: '1'
                },
                {
                    label: &quot;Viacnásobné postihnutie&quot;,
                    hint: &quot;Mentálne postihnutie v kombinácií s iným postihnutím.&quot;,
                    value: '2'
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        pokracovanieVPlneniPPV: {
            label: &quot;Pokračovanie v plnení povinného predprimárneho vzdelávania&quot;,
            hint: &quot;Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.&quot;
        },
        specialneVVP: {
            label: &quot;Má žiak špeciálne výchovno-vzdelávacie potreby?&quot;,
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        mentalnePostihnutie: {
            label: &quot;Má žiak mentálne postihnutie?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        }
    };

    const pageTextsDoplnujuceUdaje = {
        vzdelavanieVJazykuLabelMS: 'Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku',
        vzdelavanieVJazykuLabelZS: 'Vzdelávanie dieťaťa žiadam poskytovať v jazyku',
        sectionHeaderDietaText: 'Doplňujúce informácie o dieťati',
        sectionHeaderZiakText: 'Doplňujúce informácie o žiakovi',
        sectionSubHeaderDietaText: 'Vyplňte dodatočné informácie o potrebách vášho dieťaťa.',
        sectionSubHeaderZiakText: 'V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. '
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách vášho dieťaťa.
    

    

        Polia označené hviezdičkou sú povinné

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
            

    
    
        Požadovaná výchova
        (nepovinné)
    
    
    
    
                        
                        Etická 
                         
                        
                        Náboženská 
                         
    

        

        
            
            

    
    
        Vyberte typ
        (nepovinné)
    
    
    
    
                        
                        Rímskokatolícka 
                         
                        
                        Evanjelická 
                         
                        
                        Pravoslávna 
                         
                        
                        Reformovaná 
                         
                        
                        Gréckokatolícka 
                         
                        
                        Iná. Zadajte typ: 
                         
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v školskej jedálni
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Záujem o Školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Zdravotné znevýhodnenie dieťaťa
        (nepovinné)
    
    Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.
    
                        
                        Áno 
                         
            
            
                

    
        Popis znevýhodnenia
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Dieťa s nadaním
        (nepovinné)
    
    Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
                        Áno 
                         
            
            
                

    
        Popis nadania
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť?
        (nepovinné)
    
    Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte &quot;Áno&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. 
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
        

        
            

    
    
        Má žiak špeciálne výchovno-vzdelávacie potreby?
        (nepovinné)
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        So zdravotným znevýhodnením 
                         Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.  
                        
                        Zo sociálne znevýhodneného prostredia 
                         Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.  
                        
                        S nadaním 
                         Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        
            

    
    
        Má žiak mentálne postihnutie?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        Mentálne postihnutie 
                         
                        
                        Viacnásobné postihnutie 
                         Mentálne postihnutie v kombinácií s iným postihnutím.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        

    
        Požadovaný dátum prijatia dieťaťa do materskej školy
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window['pozadovanyDatumPrijatiaControlSettings'] = {
        pozadovanyDatumPrijatiaDen: {
            label: 'Deň',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'DD',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaMesiac: {
            label: 'Mesiac',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'MM',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaRok: {
            label: 'Rok',
            showRequiredOrOptional: false,
            attributes: {
                minLength: '4',
                maxLength: '4',
                placeholder: 'YYYY',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        
            

    
        Poznámka:
        (nepovinné)
        
    
    
        warning
        
        
            4/500
        
    
    Môžete uviesť doplňujúce informácie, stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka. Ak má dieťa súrodenca na škole, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.


        

        
            
            

    
    
    
        
            
                
                    Pokračovanie v plnení povinného predprimárneho vzdelávania
                    (nepovinné)
                
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

        
    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        povinnePopis: &quot;Polia označené hviezdičkou sú povinné&quot;,
        radioLabel: &quot;Kam chcete podať prihlášku?&quot;,
        optionZS: {
            label: &quot;Na základnú školu&quot;,
            value: &quot;ZŠ&quot;
        },
        optionMS:
        {
            label: &quot;Na materskú školu&quot;,
            value: &quot;MŠ&quot;
        },
        '8060': 'Podľa veku dieťaťa nie je možné podať prihlášku na materskú ani na základnú školu.',
        '8061': 'Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.',
        '8062': 'Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.',
        '8072': 'Podľa veku dieťaťa nie je možné podať prihlášku na základnú školu',
        '8073': 'Podľa veku dieťaťa nie je možné podať prihlášku na materskú školu',
        'PodanieDoMSNieJeDostupne': 'Elektronické podanie prihlášky do materskej školy nie je dostupné',
        'PodanieDoZSNieJeDostupne': 'Elektronické podanie prihlášky do základnej školy nie je dostupné',

        vyberSkolyTitle: &quot;Vybrať školy&quot;,
        vyberSkolyTitleSS: &quot;Vybrať školy a odbory&quot;,
        vyberSkolyDescription: &quot;Pridajte všetky školy, uvedené na prihláške.&quot;,
        vyberSkolyDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške a následne ich kliknutím a potiahnutím zoraďte podľa poradia v akom sú uvedené. Poradie odboru určuje jeho preferenciu.&quot;,
        vyberSkolyDescriptionSSKolo2: &quot;Pridajte odbor uvedený na prihláške.&quot;,

        vyberSkolyPridaneTitleSS: &quot;Pridané školy a odbory&quot;,
        vyberSkolyPridane0TextSS: &quot;Nie je pridaná žiadna škola ani odbor.&quot;,
        vyberSkolyPridaneDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške.&quot;,

        vyberSkolyPoradieDescription: &quot;Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.&quot;,

        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolHeader: &quot;Do prihlášky ste pridali maximálny počet škôl&quot;,
        warningMaxPocetSkolText2: &quot;Ak chcete pridať ďalšiu školu, najskôr jednu odstráňte.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;Ak chcete pridať ďalší odbor, najskôr jeden odstráňte.&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetSkolZodpovedaKriteriam: &quot; škôl zodpovedá vašim kritériám.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;&quot;,
        pocetOdborovVPrihlaskeText2: &quot;&quot;,
        pocetOdborovVPrihlaskeText5: &quot;&quot;,

        pocetOdborovText1: &quot;odbor zodpovedá vašim kritériám.&quot;,
        pocetOdborovText2: &quot;odbory zodpovedajú vašim kritériám.&quot;,
        pocetOdborovText5: &quot;odborov zodpovedá vašim kritériám.&quot;,

        jazykSelectMS: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        jazykSelectZS: {
            label: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        zaujemOUvodnyAleboPripravnyRocnik: {
            label: &quot;Mám záujem o prípravný alebo úvodný ročník&quot;,
            showRequiredOrOptional: true,
            required: false,
        },

        szsZS: {
            label: &quot;Vyberte&quot;,
            showRequiredOrOptional: true,
            required: true,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
                    value: &quot;UVODNY_ROCNIK&quot;,
                },
            ]
        },

        notSzsZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },
        notSzsNadaneZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },


        dualneVzdelavanie: {
            label: &quot;Záujem o prípravu v systéme duálneho vzdelávania&quot;,
            hint: &quot;Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.&quot;,
            showRequiredOrOptional: false,
            required: false,

        },
        internat: {
            label: &quot;Záujem o školský internát&quot;,
            showRequiredOrOptional: false,
            required: false,
        },
        terminPrijimacejSkusky: {
            label: &quot;Termín prijímacej skúšky&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]

        },
    }



    
        Vybrať školy
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        

        
            
                Pridané školy
            
            
                1
                školu ste pridali do prihlášky.
            
        

        
            Moja škola
            
                
                    Základná škola pre AT
                
                
                    EDUID 
                    910021625
                
                
                    Jalmová 266/19, 06534 Prešov
                
            
            
                

    
        Vzdelávanie dieťaťa žiadam poskytovať v jazyku
        (nepovinné)
    
    
    
        slovenský
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            
            
                
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                

                
                    

    
    
    
        
            
                
                    Záujem o úvodný ročník
                    (nepovinné)
                
                Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
            
            Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
        
    

                
            
            
                
            
        

        
            
                add
                Pridať školu
            
        

        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Základná škola pre AT
                    
                    
                        Jalmová 266/19, 06534 Prešov
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca {osoba}
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window['zastupca1DatumNarodeniaControlSettings'] = {
        zastupca1DatumNarodeniaDen: {
            label: 'Deň',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'DD',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaMesiac: {
            label: 'Mesiac',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'MM',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaRok: {
            label: 'Rok',
            showRequiredOrOptional: false,
            attributes: {
                minLength: '4',
                maxLength: '4',
                placeholder: 'YYYY',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window['zastupca1InaAdresaControlSettings'] = {
        zastupca1InaAdresaKrajina: {
            label: 'Krajina',
            hint: 'Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.',
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
        zastupca1InaAdresaObec: {
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
        zastupca1InaAdresaUlica: {
            label: 'Ulica',
            required: false,
            customOptionalText: '(ak existuje – potrebné pre doručovanie rozhodnutia)',
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
        zastupca1InaAdresaUlicaReq: {
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
        zastupca1InaAdresaSupisneCislo: {
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
        zastupca1InaAdresaOrientacneCislo: {
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
        zastupca1InaAdresaPSC: {
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
        zastupca1InaAdresaAdresa: {
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


            

            

            
                Osobné údaje zákonného zástupcu č. 2
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                
                
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                    

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window['zastupca2DatumNarodeniaControlSettings'] = {
        zastupca2DatumNarodeniaDen: {
            label: 'Deň',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'DD',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaMesiac: {
            label: 'Mesiac',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'MM',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaRok: {
            label: 'Rok',
            showRequiredOrOptional: false,
            attributes: {
                minLength: '4',
                maxLength: '4',
                placeholder: 'YYYY',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                    
                    
                        
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window['zastupca2AdresaControlSettings'] = {
        zastupca2AdresaKrajina: {
            label: 'Krajina',
            hint: 'Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.',
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
        zastupca2AdresaObec: {
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
        zastupca2AdresaUlica: {
            label: 'Ulica',
            required: false,
            customOptionalText: '(ak existuje – potrebné pre doručovanie rozhodnutia)',
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
        zastupca2AdresaUlicaReq: {
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
        zastupca2AdresaSupisneCislo: {
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
        zastupca2AdresaOrientacneCislo: {
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
        zastupca2AdresaPSC: {
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
        zastupca2AdresaAdresa: {
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


                    
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window['adresaZariadeniaControlSettings'] = {
        adresaZariadeniaKrajina: {
            label: 'Krajina',
            hint: 'Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.',
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
        adresaZariadeniaObec: {
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
        adresaZariadeniaUlica: {
            label: 'Ulica',
            required: false,
            customOptionalText: '(ak existuje – potrebné pre doručovanie rozhodnutia)',
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
        adresaZariadeniaUlicaReq: {
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
        adresaZariadeniaSupisneCislo: {
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
        adresaZariadeniaOrientacneCislo: {
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
        adresaZariadeniaPSC: {
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
        adresaZariadeniaAdresa: {
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


                        
        
    

                    
                    
                    
                    
                    
                        

    const pageTextsPrilohy = {
        '8106': 'Nahrávanie sa nepodarilo. Skúste to ešte raz.',
        '8105': 'Dokument bol úspešne nahratý.',
        '8104': 'Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.',
        '8113': 'Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.',
        '8108': 'Dokument bol úspešne vymazaný.',
        '8109': 'Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.',
        '8110': 'Všetky potrebné prílohy boli pridané.',
        'ziadnePovinnePrilohy': 'Nie sú potrebné žiadne povinné prílohy.',
        'PrilohyNahrajteYellow': 'Prosím, nahrajte všetky povinné prílohy.',
        'PrilohyNahraneYellow': 'Nahrané prílohy:',
        'PrilohyChybajuceYellow': 'Chýbajúce prílohy:',
        'PrilohyNahraneSuborySuccess': 'Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.',
        'VsetkyPotrebnePrilohyNahrane': 'Všetky potrebné prílohy boli nahrané.',
        'Nahrane0Priloh': 'Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.',
        'ChybajuPovinnePrilohy': 'Chýbajú povinné prílohy',
        'NahrajteNasledujucePrilohy': 'Nahrajte nasledujúce prílohy:',
        'kodOdboru': 'Kód odboru',
        'jazyk': 'Jazyk',
        'olympiadaSutaz': 'Olympiáda / súťaž',
        'skolskyRok': 'Školský rok',
        'druhSportu': 'Druh športu',
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect: {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        },

        modalPridatInuPrilohuNazovPrilohy: {
            label: &quot;Typ prílohy&quot;,
            required: true,
            type: &quot;input&quot;,
            name: 'inyTypPrilohy',
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;
                }

            ]
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
    

    
        
            Nahrané iné súbory
        
        
        
    

    

    
        add
        Pridať inú prílohu
    

    
    

    
    

    
        
            Zvoľte, ako chcete nahrať súbor:
            Súbory
            Knižnica fotografií
            Fotoaparát
        
        Cancel
    

    

    

    


                    
                    
                        

    const controlSettingsOU = {
        aktualnyRokValidationErrorMessage: 'Zadajte dátum z aktuálneho roka.',
        rokVBuducnostiValidationErrorMessage: 'Zadaný dátum musí by aktuálny dátum alebo starší ako aktuálny dátum.',
        tvarDatumuValidationErrorMessage: 'Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.08.2025).',
        povinneErrorMessage: 'Toto pole je povinné.',
        datumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
             validators: []
        },
        OUPoznamkaText: {
            label: &quot;Poznámka školy:&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },

        '8084': 'Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.',

    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    Polia označené hviezdičkou sú povinné

    
        Dátum prihlášky

        

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window['datumPodaniaPrihlaskyControlSettings'] = {
        datumPodaniaPrihlaskyDen: {
            label: 'Deň',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'DD',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyMesiac: {
            label: 'Mesiac',
            showRequiredOrOptional: false,
            attributes: {
                maxLength: '2',
                placeholder: 'MM',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyRok: {
            label: 'Rok',
            showRequiredOrOptional: false,
            attributes: {
                minLength: '4',
                maxLength: '4',
                placeholder: 'YYYY',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        Poznámka k prihláške

        
            

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        dietaHeaderMS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderZS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderSS: &quot;Osobné údaje žiaka&quot;,
        skolyHeaderMS: &quot;Materská škola&quot;,
        skolyHeaderZS: &quot;Základná škola&quot;,
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaInfoHeaderMS: &quot;Materská škola č.&quot;,
        skolaInfoHeaderZS: &quot;Základná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;Poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;Celodennú výchovu a vzdelávanie&quot;,
        spravaDovod: &quot;&quot;,
        spravaPrilozeneDokumenty: &quot;&quot;,
        nastupPotvrdeny: &quot;&quot;,
        nenastupi: &quot;&quot;,
        identifikator: &quot;&quot;,
        dietati: 'dieťati',
        dietata: 'dieťaťa',
        ziakovi: 'žiakovi',
        ziaka: 'žiaka',
        zdravotneZnevyhodnenie: &quot;So zdravotným znevýhodnením&quot;,
        socialneZnevyhodnenie: &quot;Zo sociálne znevýhodneného prostredia&quot;,
        nadanie: &quot;S nadaním&quot;,
        mentalne: &quot;Mentálne zdravotné postihnutie&quot;,
        mentalneSInym: &quot;Viacnásobné postihnutie&quot;,
        kolo: &quot;{kolo}. kolo&quot;,
        skolskyRok: &quot;Školský rok:&quot;,
        uroven: &quot;úroveň&quot;,
        prichodZoZSNaSVK: &quot;Zo ZŠ na Slovensku&quot;,
        prichodZoZahranicia: &quot;Zo školy v zahraničí&quot;,
        zPraxe: &quot;Z praxe&quot;,
        ine: &quot;Iné&quot;,
        eduidSkoly: &quot;EDUID školy&quot;,
        nazovStrednejSkoly: &quot;Názov strednej školy&quot;,
        kodOdboru: &quot;Kód odboru&quot;,
        nazovOdboru: &quot;Názov odboru&quot;,
        typOdboru: &quot;Typ odboru&quot;,
        terminPrijimacejSkusky: &quot;Termín prijímacej skúšky&quot;,
        vyucovaciJazykOdboru: &quot;Vyučovací jazyk odboru&quot;,
        zaujemODualneVzdelavanie: &quot;Záujem o duálne vzdelávanie&quot;,
        zaujemOSkolskyInternat: &quot;Záujem o školský internát&quot;,
        talentovy: &quot;Talentový&quot;,
        netalentovy: &quot;Netalentový&quot;,
        zastupcoviaHeader: &quot;Osobné údaje zákonných zástupcov žiaka&quot;,
        zastupcoviaHeaderMSZS: &quot;Osobné údaje zákonných zástupcov dieťaťa&quot;,
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        sidloMS: &quot;Sídlo materskej školy&quot;,
        sidloZS: &quot;Sídlo základnej školy&quot;,
        vzdelavanieVJazykuLabelZS: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
        zaujemOPripravnyRocnik: &quot;Záujem o prípravný ročník&quot;,
        zaujemOUvodnyRocnik: &quot;Záujem o úvodný ročník&quot;,
        predprimarneVzdelavanie: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href='https://www.slovensko.sk/' target='_blank'>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }

    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        
            
                Všeobecné informácie
            
            Upraviť
        
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Poznámka školy
                -
            
            
                Kolo prijímacieho konania
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
            Upraviť
        
        
            
                Meno
                -
            
            
                Priezvisko
                -
            
            
                Rodné priezvisko
                -
            
            
                Rodné číslo
                -
            
            
                Dátum narodenia
                -
            
            
                Pohlavie
                -
            
            
                Miesto narodenia
                -
            
            
                Národnosť
                -
            
            
                Štátna príslušnosť
                -
            
            
                Materinský jazyk
                -
            
            
                Iný materinský jazyk
                -
            
            
                Adresa trvalého pobytu
                -
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v školskej jedálni
                        
                    
                    
                        Záujem o školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa
                        

                    
                    
                        Dieťa s nadaním
                        
                    
                    
                        Požadovaný dátum prijatia dieťaťa do materskej školy
                        -
                    
                    
                        Popis znevýhodnenia
                        
                    
                    
                        Popis nadania
                        
                    
                    
                        Poznámka
                        
                    
                    
                        Pokračovanie v plnení povinného predprimárneho vzdelávania
                        -
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Špeciálne výchovno-vzdelávacie potreby
                        -
                    
                    
                        Mentálne postihnutie
                        -
                    
                    
                        
                        
                    
                    
                        Poznámka
                        -
                    
                
            
        
    

    
        
            
                Stredná škola
            
            Upraviť
        
        
        
    

    
        
            
            
            Upraviť
        
        
            
                Osobné údaje zákonného zástupcu č. 1
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas s komunikáciou výhradne so zákonným zástupcom č. 1
                        -
                    
                
                
                Osobné údaje zákonného zástupcu č. 2
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca dieťaťa
                        -
                    
                    
                        Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                        -
                    
                
                Druhý zákonný zástupca nie je známy.
            
            
                
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                    
                        Adresa zariadenia
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                
            
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                -
            
            
                EDUID základnej školy
                -
            
            
                Názov základnej školy
                -
            
            
                Ročník
                -
            
            
                Trieda
                -
            
            
                Rok školskej dochádzky
                -
            
            
                Vyučovací jazyk v základnej škole
                -
            
        
    

    
        
            
                Výsledky vzdelávania na základnej škole
            
            Upraviť
        
        
        
    

    
        
            
                Súťaže
            
            Upraviť
        
        
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            
            
        
    

    
        
            
                
            
            
                Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
        
    

    





EXPORT PDF

                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                </value>
      <webElementGuid>b752bb20-5d40-4441-8469-5f9d0486812d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ziadost&quot;)/div[@class=&quot;menu-layout&quot;]/div[@class=&quot;privatna-zona-content&quot;]/div[@class=&quot;govuk-form-group&quot;]/div[@class=&quot;govuk-wizard-steps&quot;]</value>
      <webElementGuid>1a3c5a84-2967-4869-834f-ea2a0c1d5bc9</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='ziadost']/div/div/div/div</value>
      <webElementGuid>324dfd16-7adc-436d-ab0d-2a471ef87bfa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Odhlásiť'])[2]/following::div[5]</value>
      <webElementGuid>980cd179-50e9-4a9d-ba4e-8e3d815826d2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Oznámenia'])[2]/following::div[5]</value>
      <webElementGuid>f6664657-3356-457f-9d94-5231436133d2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body/div[2]/div/div/div/div</value>
      <webElementGuid>7749148c-82fa-44c5-8ffc-02f83e025a28</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                    

    
                        
                            Správa prihlášok
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    
                    
                        
                            
                                Krok3/7
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje dieťaťa
                        
                    
                        
                            2.
                            Doplňujúce údaje o dieťati
                        
                    
                        
                            3.
                            Vybrať školy
                        
                    
                        
                            4.
                            Zákonný zástupca dieťaťa
                        
                    
                        
                            5.
                            Pridať prílohy
                        
                    
                        
                            6.
                            Ostatné údaje
                        
                    
                        
                            7.
                            Súhrnný prehľad
                        
                    
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: &quot; , &quot;'&quot; , &quot;Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne&quot; , &quot;'&quot; , &quot;,
        prosimCakajte: &quot; , &quot;'&quot; , &quot;Prosím, čakajte&quot; , &quot;'&quot; , &quot;,
        systemPracuje: &quot; , &quot;'&quot; , &quot;Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;dietaNemaInyMaterinsky&quot; , &quot;'&quot; , &quot;: &quot;Dieťa nemá iný materinský jazyk&quot;,
         inaAdresaTP: &quot;Iná adresa trvalého pobytu&quot;,
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má {osoba} rodné číslo?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Nie&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Uveďte rodné číslo dieťaťa. Ak dieťa rodné číslo nemá, zvoľte možnosť \&quot;Nie\&quot;.&quot;,
        },
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
        },
        krstneMeno: {
            label: &quot;Meno&quot;,
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
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
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
            label: &quot;Adresa trvalého pobytu {osoba}&quot;,
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
            ],
        },
        zvycajnaAdresaradio: {
            label: &quot;Vyberte adresu&quot;,
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
        plnolety: {
            label: &quot;Uchádzač je plnoletý&quot;,
            element: &quot; , &quot;'&quot; , &quot;plnolety&quot; , &quot;'&quot; , &quot;,
            ano: &quot;Áno&quot;,
            nie: &quot;Nie&quot;,
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    Osobné údaje dieťaťa

    
        
            Pridať dieťa podľa rodného čísla.
        
        Polia označené hviezdičkou sú povinné
        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            
                        
                        Nie 
                         
    

            
            
        
    
    
        

            
                Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.
            

            Polia označené hviezdičkou sú povinné

            
                
                    
                        Osobné údaje dieťaťa
                    
                
                
                    
                        Meno
                        Tibor
                    
                    
                        Priezvisko
                        Lesko
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        200112/6853
                    
                    
                        Dátum narodenia
                        12.01.2020
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Slovensko
                    
                    
                        Národnosť
                        slovenská
                    
                    
                        Štátna príslušnosť
                        Slovenská republika
                    
                    
                        Materinský jazyk
                        slovenský
                    
                    
                        Iný materinský jazyk
                        -
                    
                    
                        Uchádzač je plnoletý
                        -
                    
                    
                        Adresa trvalého pobytu
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Predprimárne vzdelávanie
                        -
                    
                
            

            
                
                    
                        Osobné údaje dieťaťa
                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window[&quot; , &quot;'&quot; , &quot;datumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        datumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
    
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
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        ##### 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    Slovenská republikaSlovenská republika – mimo číselník


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Predmier (Bytča)
        warning
        
            keyboard_arrow_down
        
    Predmier (Bytča)
Toto pole je povinné.

    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        Egrešská
        warning
        
            keyboard_arrow_down
        
    Egrešská


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaTPControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaTPKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaZAControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZAKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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


                        
                    
                    
                        

    
    
    
        
            
                
                    Uchádzač je plnoletý
                    (nepovinné)
                
                
            
            
        
    

                    
                

                
                    Predprimárne vzdelávanie
                    
                        

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

            
        
    


                    
                    
                        

    const controlSettingsDPD = {
        msCelodennaVychovaRadio : {
            label: &quot;Žiadam o prijatie dieťaťa na&quot;,
            options: [
                {
                    label: &quot;poldennú výchovu a vzdelávanie&quot;,
                    value: false
                },
                {
                    label: &quot;celodennú výchovu a vzdelávanie&quot;,
                    value: true
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        DPDSVVPRadio: {
            label: &quot;Zdravotné znevýhodnenie dieťaťa&quot;,
            hint: &quot;Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDDietaSNadanimRadio: {
            label: &quot;Dieťa s nadaním&quot;,
            hint: &quot;Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        zsDPDVychovaRadio : {
            label: &quot;Požadovaná výchova&quot;,
            options: [
                {
                    label: &quot;Etická&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Náboženská&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiRadio : {
            label: &quot;Vyberte typ&quot;,
            options: [
                {
                    label: &quot;Rímskokatolícka&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Rímskokatolícka&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Evanjelická&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Evanjelická&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Pravoslávna&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Pravoslávna&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Reformovaná&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Reformovaná&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Gréckokatolícka&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Gréckokatolícka&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná. Zadajte typ:&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Iná&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]
        },
        DPDPopisNadaniaText: {
            label: &quot;Popis nadania&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v školskej jedálni&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDSkolskyKlubRadio : {
            label: &quot;Záujem o Školský klub detí&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            hintZsMs: &quot;Môžete uviesť doplňujúce informácie, &lt;b>stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti&lt;/b>, ktoré môžu ovplyvniť vzdelávanie žiaka. &lt;br>Ak má dieťa &lt;b>súrodenca na škole&lt;/b>, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
            anyDate: true,
            validators: [],
            povinneError: &quot;Toto pole je povinné.&quot;
        },
        zmenenaPracovnaSchopnostRadio : {
            label: &quot;Má žiak zmenenú pracovnú schopnosť?&quot;,
            hint: &quot;Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte \&quot;Áno\&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        specialneVVPRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;So zdravotným znevýhodnením&quot;,
                    hint: &quot;Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Zo sociálne znevýhodneného prostredia&quot;,
                    hint: &quot;Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;S nadaním&quot;,
                    hint: &quot;Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        mentalnePostihnutieRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;Mentálne postihnutie&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Viacnásobné postihnutie&quot;,
                    hint: &quot;Mentálne postihnutie v kombinácií s iným postihnutím.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        pokracovanieVPlneniPPV: {
            label: &quot;Pokračovanie v plnení povinného predprimárneho vzdelávania&quot;,
            hint: &quot;Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.&quot;
        },
        specialneVVP: {
            label: &quot;Má žiak špeciálne výchovno-vzdelávacie potreby?&quot;,
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        mentalnePostihnutie: {
            label: &quot;Má žiak mentálne postihnutie?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        }
    };

    const pageTextsDoplnujuceUdaje = {
        vzdelavanieVJazykuLabelMS: &quot; , &quot;'&quot; , &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot; , &quot;'&quot; , &quot;,
        vzdelavanieVJazykuLabelZS: &quot; , &quot;'&quot; , &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot; , &quot;'&quot; , &quot;,
        sectionHeaderDietaText: &quot; , &quot;'&quot; , &quot;Doplňujúce informácie o dieťati&quot; , &quot;'&quot; , &quot;,
        sectionHeaderZiakText: &quot; , &quot;'&quot; , &quot;Doplňujúce informácie o žiakovi&quot; , &quot;'&quot; , &quot;,
        sectionSubHeaderDietaText: &quot; , &quot;'&quot; , &quot;Vyplňte dodatočné informácie o potrebách vášho dieťaťa.&quot; , &quot;'&quot; , &quot;,
        sectionSubHeaderZiakText: &quot; , &quot;'&quot; , &quot;V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. &quot; , &quot;'&quot; , &quot;
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách vášho dieťaťa.
    

    

        Polia označené hviezdičkou sú povinné

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
            

    
    
        Požadovaná výchova
        (nepovinné)
    
    
    
    
                        
                        Etická 
                         
                        
                        Náboženská 
                         
    

        

        
            
            

    
    
        Vyberte typ
        (nepovinné)
    
    
    
    
                        
                        Rímskokatolícka 
                         
                        
                        Evanjelická 
                         
                        
                        Pravoslávna 
                         
                        
                        Reformovaná 
                         
                        
                        Gréckokatolícka 
                         
                        
                        Iná. Zadajte typ: 
                         
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v školskej jedálni
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Záujem o Školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Zdravotné znevýhodnenie dieťaťa
        (nepovinné)
    
    Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.
    
                        
                        Áno 
                         
            
            
                

    
        Popis znevýhodnenia
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Dieťa s nadaním
        (nepovinné)
    
    Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
                        Áno 
                         
            
            
                

    
        Popis nadania
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť?
        (nepovinné)
    
    Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte &quot;Áno&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. 
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
        

        
            

    
    
        Má žiak špeciálne výchovno-vzdelávacie potreby?
        (nepovinné)
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        So zdravotným znevýhodnením 
                         Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.  
                        
                        Zo sociálne znevýhodneného prostredia 
                         Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.  
                        
                        S nadaním 
                         Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        
            

    
    
        Má žiak mentálne postihnutie?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        Mentálne postihnutie 
                         
                        
                        Viacnásobné postihnutie 
                         Mentálne postihnutie v kombinácií s iným postihnutím.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        

    
        Požadovaný dátum prijatia dieťaťa do materskej školy
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window[&quot; , &quot;'&quot; , &quot;pozadovanyDatumPrijatiaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        pozadovanyDatumPrijatiaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        
            

    
        Poznámka:
        (nepovinné)
        
    
    
        warning
        
        
            4/500
        
    
    Môžete uviesť doplňujúce informácie, stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka. Ak má dieťa súrodenca na škole, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.


        

        
            
            

    
    
    
        
            
                
                    Pokračovanie v plnení povinného predprimárneho vzdelávania
                    (nepovinné)
                
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

        
    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        povinnePopis: &quot;Polia označené hviezdičkou sú povinné&quot;,
        radioLabel: &quot;Kam chcete podať prihlášku?&quot;,
        optionZS: {
            label: &quot;Na základnú školu&quot;,
            value: &quot;ZŠ&quot;
        },
        optionMS:
        {
            label: &quot;Na materskú školu&quot;,
            value: &quot;MŠ&quot;
        },
        &quot; , &quot;'&quot; , &quot;8060&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na materskú ani na základnú školu.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8061&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8062&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8072&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na základnú školu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8073&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na materskú školu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PodanieDoMSNieJeDostupne&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Elektronické podanie prihlášky do materskej školy nie je dostupné&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PodanieDoZSNieJeDostupne&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Elektronické podanie prihlášky do základnej školy nie je dostupné&quot; , &quot;'&quot; , &quot;,

        vyberSkolyTitle: &quot;Vybrať školy&quot;,
        vyberSkolyTitleSS: &quot;Vybrať školy a odbory&quot;,
        vyberSkolyDescription: &quot;Pridajte všetky školy, uvedené na prihláške.&quot;,
        vyberSkolyDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške a následne ich kliknutím a potiahnutím zoraďte podľa poradia v akom sú uvedené. Poradie odboru určuje jeho preferenciu.&quot;,
        vyberSkolyDescriptionSSKolo2: &quot;Pridajte odbor uvedený na prihláške.&quot;,

        vyberSkolyPridaneTitleSS: &quot;Pridané školy a odbory&quot;,
        vyberSkolyPridane0TextSS: &quot;Nie je pridaná žiadna škola ani odbor.&quot;,
        vyberSkolyPridaneDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške.&quot;,

        vyberSkolyPoradieDescription: &quot;Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.&quot;,

        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolHeader: &quot;Do prihlášky ste pridali maximálny počet škôl&quot;,
        warningMaxPocetSkolText2: &quot;Ak chcete pridať ďalšiu školu, najskôr jednu odstráňte.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;Ak chcete pridať ďalší odbor, najskôr jeden odstráňte.&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetSkolZodpovedaKriteriam: &quot; škôl zodpovedá vašim kritériám.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;&quot;,
        pocetOdborovVPrihlaskeText2: &quot;&quot;,
        pocetOdborovVPrihlaskeText5: &quot;&quot;,

        pocetOdborovText1: &quot;odbor zodpovedá vašim kritériám.&quot;,
        pocetOdborovText2: &quot;odbory zodpovedajú vašim kritériám.&quot;,
        pocetOdborovText5: &quot;odborov zodpovedá vašim kritériám.&quot;,

        jazykSelectMS: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        jazykSelectZS: {
            label: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        zaujemOUvodnyAleboPripravnyRocnik: {
            label: &quot;Mám záujem o prípravný alebo úvodný ročník&quot;,
            showRequiredOrOptional: true,
            required: false,
        },

        szsZS: {
            label: &quot;Vyberte&quot;,
            showRequiredOrOptional: true,
            required: true,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
                    value: &quot;UVODNY_ROCNIK&quot;,
                },
            ]
        },

        notSzsZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },
        notSzsNadaneZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },


        dualneVzdelavanie: {
            label: &quot;Záujem o prípravu v systéme duálneho vzdelávania&quot;,
            hint: &quot;Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.&quot;,
            showRequiredOrOptional: false,
            required: false,

        },
        internat: {
            label: &quot;Záujem o školský internát&quot;,
            showRequiredOrOptional: false,
            required: false,
        },
        terminPrijimacejSkusky: {
            label: &quot;Termín prijímacej skúšky&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]

        },
    }



    
        Vybrať školy
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        

        
            
                Pridané školy
            
            
                1
                školu ste pridali do prihlášky.
            
        

        
            Moja škola
            
                
                    Základná škola pre AT
                
                
                    EDUID 
                    910021625
                
                
                    Jalmová 266/19, 06534 Prešov
                
            
            
                

    
        Vzdelávanie dieťaťa žiadam poskytovať v jazyku
        (nepovinné)
    
    
    
        slovenský
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            
            
                
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                

                
                    

    
    
    
        
            
                
                    Záujem o úvodný ročník
                    (nepovinné)
                
                Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
            
            Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
        
    

                
            
            
                
            
        

        
            
                add
                Pridať školu
            
        

        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Základná škola pre AT
                    
                    
                        Jalmová 266/19, 06534 Prešov
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca {osoba}
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;zastupca1DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1DatumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1InaAdresaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        zastupca1InaAdresaObec: {
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
        zastupca1InaAdresaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        zastupca1InaAdresaUlicaReq: {
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
        zastupca1InaAdresaSupisneCislo: {
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
        zastupca1InaAdresaOrientacneCislo: {
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
        zastupca1InaAdresaPSC: {
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
        zastupca1InaAdresaAdresa: {
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


            

            

            
                Osobné údaje zákonného zástupcu č. 2
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                
                
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                    

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;zastupca2DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2DatumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                    
                    
                        
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;zastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2AdresaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        zastupca2AdresaObec: {
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
        zastupca2AdresaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        zastupca2AdresaUlicaReq: {
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
        zastupca2AdresaSupisneCislo: {
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
        zastupca2AdresaOrientacneCislo: {
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
        zastupca2AdresaPSC: {
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
        zastupca2AdresaAdresa: {
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


                    
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;adresaZariadeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZariadeniaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        adresaZariadeniaObec: {
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
        adresaZariadeniaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        adresaZariadeniaUlicaReq: {
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
        adresaZariadeniaSupisneCislo: {
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
        adresaZariadeniaOrientacneCislo: {
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
        adresaZariadeniaPSC: {
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
        adresaZariadeniaAdresa: {
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


                        
        
    

                    
                    
                    
                    
                    
                        

    const pageTextsPrilohy = {
        &quot; , &quot;'&quot; , &quot;8106&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8105&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne nahratý.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8104&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8113&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8108&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne vymazaný.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8109&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8110&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli pridané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ziadnePovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nie sú potrebné žiadne povinné prílohy.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahrajteYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Prosím, nahrajte všetky povinné prílohy.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahraneYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrané prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyChybajuceYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Chýbajúce prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahraneSuborySuccess&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;VsetkyPotrebnePrilohyNahrane&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli nahrané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;Nahrane0Priloh&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ChybajuPovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Chýbajú povinné prílohy&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NahrajteNasledujucePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrajte nasledujúce prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;kodOdboru&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Kód odboru&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;jazyk&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Jazyk&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;olympiadaSutaz&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Olympiáda / súťaž&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;skolskyRok&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Školský rok&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;druhSportu&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Druh športu&quot; , &quot;'&quot; , &quot;,
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect: {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        },

        modalPridatInuPrilohuNazovPrilohy: {
            label: &quot;Typ prílohy&quot;,
            required: true,
            type: &quot;input&quot;,
            name: &quot; , &quot;'&quot; , &quot;inyTypPrilohy&quot; , &quot;'&quot; , &quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }

            ]
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
    

    
        
            Nahrané iné súbory
        
        
        
    

    

    
        add
        Pridať inú prílohu
    

    
    

    
    

    
        
            Zvoľte, ako chcete nahrať súbor:
            Súbory
            Knižnica fotografií
            Fotoaparát
        
        Cancel
    

    

    

    


                    
                    
                        

    const controlSettingsOU = {
        aktualnyRokValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Zadajte dátum z aktuálneho roka.&quot; , &quot;'&quot; , &quot;,
        rokVBuducnostiValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Zadaný dátum musí by aktuálny dátum alebo starší ako aktuálny dátum.&quot; , &quot;'&quot; , &quot;,
        tvarDatumuValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.08.2025).&quot; , &quot;'&quot; , &quot;,
        povinneErrorMessage: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;,
        datumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
             validators: []
        },
        OUPoznamkaText: {
            label: &quot;Poznámka školy:&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },

        &quot; , &quot;'&quot; , &quot;8084&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.&quot; , &quot;'&quot; , &quot;,

    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    Polia označené hviezdičkou sú povinné

    
        Dátum prihlášky

        

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;datumPodaniaPrihlaskyControlSettings&quot; , &quot;'&quot; , &quot;] = {
        datumPodaniaPrihlaskyDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        Poznámka k prihláške

        
            

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        dietaHeaderMS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderZS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderSS: &quot;Osobné údaje žiaka&quot;,
        skolyHeaderMS: &quot;Materská škola&quot;,
        skolyHeaderZS: &quot;Základná škola&quot;,
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaInfoHeaderMS: &quot;Materská škola č.&quot;,
        skolaInfoHeaderZS: &quot;Základná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;Poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;Celodennú výchovu a vzdelávanie&quot;,
        spravaDovod: &quot;&quot;,
        spravaPrilozeneDokumenty: &quot;&quot;,
        nastupPotvrdeny: &quot;&quot;,
        nenastupi: &quot;&quot;,
        identifikator: &quot;&quot;,
        dietati: &quot; , &quot;'&quot; , &quot;dieťati&quot; , &quot;'&quot; , &quot;,
        dietata: &quot; , &quot;'&quot; , &quot;dieťaťa&quot; , &quot;'&quot; , &quot;,
        ziakovi: &quot; , &quot;'&quot; , &quot;žiakovi&quot; , &quot;'&quot; , &quot;,
        ziaka: &quot; , &quot;'&quot; , &quot;žiaka&quot; , &quot;'&quot; , &quot;,
        zdravotneZnevyhodnenie: &quot;So zdravotným znevýhodnením&quot;,
        socialneZnevyhodnenie: &quot;Zo sociálne znevýhodneného prostredia&quot;,
        nadanie: &quot;S nadaním&quot;,
        mentalne: &quot;Mentálne zdravotné postihnutie&quot;,
        mentalneSInym: &quot;Viacnásobné postihnutie&quot;,
        kolo: &quot;{kolo}. kolo&quot;,
        skolskyRok: &quot;Školský rok:&quot;,
        uroven: &quot;úroveň&quot;,
        prichodZoZSNaSVK: &quot;Zo ZŠ na Slovensku&quot;,
        prichodZoZahranicia: &quot;Zo školy v zahraničí&quot;,
        zPraxe: &quot;Z praxe&quot;,
        ine: &quot;Iné&quot;,
        eduidSkoly: &quot;EDUID školy&quot;,
        nazovStrednejSkoly: &quot;Názov strednej školy&quot;,
        kodOdboru: &quot;Kód odboru&quot;,
        nazovOdboru: &quot;Názov odboru&quot;,
        typOdboru: &quot;Typ odboru&quot;,
        terminPrijimacejSkusky: &quot;Termín prijímacej skúšky&quot;,
        vyucovaciJazykOdboru: &quot;Vyučovací jazyk odboru&quot;,
        zaujemODualneVzdelavanie: &quot;Záujem o duálne vzdelávanie&quot;,
        zaujemOSkolskyInternat: &quot;Záujem o školský internát&quot;,
        talentovy: &quot;Talentový&quot;,
        netalentovy: &quot;Netalentový&quot;,
        zastupcoviaHeader: &quot;Osobné údaje zákonných zástupcov žiaka&quot;,
        zastupcoviaHeaderMSZS: &quot;Osobné údaje zákonných zástupcov dieťaťa&quot;,
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        sidloMS: &quot;Sídlo materskej školy&quot;,
        sidloZS: &quot;Sídlo základnej školy&quot;,
        vzdelavanieVJazykuLabelZS: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
        zaujemOPripravnyRocnik: &quot;Záujem o prípravný ročník&quot;,
        zaujemOUvodnyRocnik: &quot;Záujem o úvodný ročník&quot;,
        predprimarneVzdelavanie: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot; target=&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }

    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        
            
                Všeobecné informácie
            
            Upraviť
        
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Poznámka školy
                -
            
            
                Kolo prijímacieho konania
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
            Upraviť
        
        
            
                Meno
                -
            
            
                Priezvisko
                -
            
            
                Rodné priezvisko
                -
            
            
                Rodné číslo
                -
            
            
                Dátum narodenia
                -
            
            
                Pohlavie
                -
            
            
                Miesto narodenia
                -
            
            
                Národnosť
                -
            
            
                Štátna príslušnosť
                -
            
            
                Materinský jazyk
                -
            
            
                Iný materinský jazyk
                -
            
            
                Adresa trvalého pobytu
                -
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v školskej jedálni
                        
                    
                    
                        Záujem o školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa
                        

                    
                    
                        Dieťa s nadaním
                        
                    
                    
                        Požadovaný dátum prijatia dieťaťa do materskej školy
                        -
                    
                    
                        Popis znevýhodnenia
                        
                    
                    
                        Popis nadania
                        
                    
                    
                        Poznámka
                        
                    
                    
                        Pokračovanie v plnení povinného predprimárneho vzdelávania
                        -
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Špeciálne výchovno-vzdelávacie potreby
                        -
                    
                    
                        Mentálne postihnutie
                        -
                    
                    
                        
                        
                    
                    
                        Poznámka
                        -
                    
                
            
        
    

    
        
            
                Stredná škola
            
            Upraviť
        
        
        
    

    
        
            
            
            Upraviť
        
        
            
                Osobné údaje zákonného zástupcu č. 1
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas s komunikáciou výhradne so zákonným zástupcom č. 1
                        -
                    
                
                
                Osobné údaje zákonného zástupcu č. 2
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca dieťaťa
                        -
                    
                    
                        Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                        -
                    
                
                Druhý zákonný zástupca nie je známy.
            
            
                
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                    
                        Adresa zariadenia
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                
            
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                -
            
            
                EDUID základnej školy
                -
            
            
                Názov základnej školy
                -
            
            
                Ročník
                -
            
            
                Trieda
                -
            
            
                Rok školskej dochádzky
                -
            
            
                Vyučovací jazyk v základnej škole
                -
            
        
    

    
        
            
                Výsledky vzdelávania na základnej škole
            
            Upraviť
        
        
        
    

    
        
            
                Súťaže
            
            Upraviť
        
        
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            
            
        
    

    
        
            
                
            
            
                Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
        
    

    





EXPORT PDF

                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                &quot;) or . = concat(&quot;
                    

    
                        
                            Správa prihlášok
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    
                    
                        
                            
                                Krok3/7
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje dieťaťa
                        
                    
                        
                            2.
                            Doplňujúce údaje o dieťati
                        
                    
                        
                            3.
                            Vybrať školy
                        
                    
                        
                            4.
                            Zákonný zástupca dieťaťa
                        
                    
                        
                            5.
                            Pridať prílohy
                        
                    
                        
                            6.
                            Ostatné údaje
                        
                    
                        
                            7.
                            Súhrnný prehľad
                        
                    
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: &quot; , &quot;'&quot; , &quot;Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne&quot; , &quot;'&quot; , &quot;,
        prosimCakajte: &quot; , &quot;'&quot; , &quot;Prosím, čakajte&quot; , &quot;'&quot; , &quot;,
        systemPracuje: &quot; , &quot;'&quot; , &quot;Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;dietaNemaInyMaterinsky&quot; , &quot;'&quot; , &quot;: &quot;Dieťa nemá iný materinský jazyk&quot;,
         inaAdresaTP: &quot;Iná adresa trvalého pobytu&quot;,
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má {osoba} rodné číslo?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Nie&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true,
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Uveďte rodné číslo dieťaťa. Ak dieťa rodné číslo nemá, zvoľte možnosť \&quot;Nie\&quot;.&quot;,
        },
        rodneCislo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
            label: &quot;Rodné číslo&quot;,
            hint: &quot;Zadajte vo formáte XXXXXX/XXXX&quot;,
        },
        krstneMeno: {
            label: &quot;Meno&quot;,
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
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
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
            label: &quot;Adresa trvalého pobytu {osoba}&quot;,
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
            ],
        },
        zvycajnaAdresaradio: {
            label: &quot;Vyberte adresu&quot;,
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
        plnolety: {
            label: &quot;Uchádzač je plnoletý&quot;,
            element: &quot; , &quot;'&quot; , &quot;plnolety&quot; , &quot;'&quot; , &quot;,
            ano: &quot;Áno&quot;,
            nie: &quot;Nie&quot;,
        },
        predprimarneVzdelavanie: {
            label: &quot;Povinné predprimárne vzdelávanie aktuálne v&quot;,
            hint: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
            regexError: &quot;Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.&quot;,
        },
    };




    Osobné údaje dieťaťa

    
        
            Pridať dieťa podľa rodného čísla.
        
        Polia označené hviezdičkou sú povinné
        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            
                        
                        Nie 
                         
    

            
            
        
    
    
        

            
                Vyplňte údaje tak, aby sa zhodovali s údajmi uvedenými v papierovej prihláške.
            

            Polia označené hviezdičkou sú povinné

            
                
                    
                        Osobné údaje dieťaťa
                    
                
                
                    
                        Meno
                        Tibor
                    
                    
                        Priezvisko
                        Lesko
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        200112/6853
                    
                    
                        Dátum narodenia
                        12.01.2020
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Slovensko
                    
                    
                        Národnosť
                        slovenská
                    
                    
                        Štátna príslušnosť
                        Slovenská republika
                    
                    
                        Materinský jazyk
                        slovenský
                    
                    
                        Iný materinský jazyk
                        -
                    
                    
                        Uchádzač je plnoletý
                        -
                    
                    
                        Adresa trvalého pobytu
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                        Egrešská 12/1, 01589, Predmier (Bytča), Slovenská republika
                    
                    
                        Predprimárne vzdelávanie
                        -
                    
                
            

            
                
                    
                        Osobné údaje dieťaťa
                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Rodné priezvisko
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
        Dátum narodenia
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window[&quot; , &quot;'&quot; , &quot;datumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        datumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
    
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
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        ##### 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    Slovenská republikaSlovenská republika – mimo číselník


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Predmier (Bytča)
        warning
        
            keyboard_arrow_down
        
    Predmier (Bytča)
Toto pole je povinné.

    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        Egrešská
        warning
        
            keyboard_arrow_down
        
    Egrešská


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaTPControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaTPKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (ak existuje – potrebné pre doručovanie rozhodnutia)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        Súpisné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        Orientačné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        PSČ
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;adresaZAControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZAKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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


                        
                    
                    
                        

    
    
    
        
            
                
                    Uchádzač je plnoletý
                    (nepovinné)
                
                
            
            
        
    

                    
                

                
                    Predprimárne vzdelávanie
                    
                        

    
        Povinné predprimárne vzdelávanie aktuálne v
        (nepovinné)
    
    Zadajte názov predprimárneho vzdelávacieho zariadenia, ktoré dieťa aktuálne navštevuje.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

            
        
    


                    
                    
                        

    const controlSettingsDPD = {
        msCelodennaVychovaRadio : {
            label: &quot;Žiadam o prijatie dieťaťa na&quot;,
            options: [
                {
                    label: &quot;poldennú výchovu a vzdelávanie&quot;,
                    value: false
                },
                {
                    label: &quot;celodennú výchovu a vzdelávanie&quot;,
                    value: true
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        DPDSVVPRadio: {
            label: &quot;Zdravotné znevýhodnenie dieťaťa&quot;,
            hint: &quot;Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDDietaSNadanimRadio: {
            label: &quot;Dieťa s nadaním&quot;,
            hint: &quot;Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        zsDPDVychovaRadio : {
            label: &quot;Požadovaná výchova&quot;,
            options: [
                {
                    label: &quot;Etická&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Náboženská&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiRadio : {
            label: &quot;Vyberte typ&quot;,
            options: [
                {
                    label: &quot;Rímskokatolícka&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Rímskokatolícka&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Evanjelická&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Evanjelická&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Pravoslávna&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Pravoslávna&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Reformovaná&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Reformovaná&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Gréckokatolícka&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Gréckokatolícka&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iná. Zadajte typ:&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Iná&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]
        },
        DPDPopisNadaniaText: {
            label: &quot;Popis nadania&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v školskej jedálni&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDSkolskyKlubRadio : {
            label: &quot;Záujem o Školský klub detí&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia&quot;,
            required: true,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            hintZsMs: &quot;Môžete uviesť doplňujúce informácie, &lt;b>stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti&lt;/b>, ktoré môžu ovplyvniť vzdelávanie žiaka. &lt;br>Ak má dieťa &lt;b>súrodenca na škole&lt;/b>, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
            anyDate: true,
            validators: [],
            povinneError: &quot;Toto pole je povinné.&quot;
        },
        zmenenaPracovnaSchopnostRadio : {
            label: &quot;Má žiak zmenenú pracovnú schopnosť?&quot;,
            hint: &quot;Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte \&quot;Áno\&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        specialneVVPRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;So zdravotným znevýhodnením&quot;,
                    hint: &quot;Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Zo sociálne znevýhodneného prostredia&quot;,
                    hint: &quot;Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;S nadaním&quot;,
                    hint: &quot;Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        mentalnePostihnutieRadio : {
            label: &quot;Vyberte&quot;,
            options: [
                {
                    label: &quot;Mentálne postihnutie&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Viacnásobné postihnutie&quot;,
                    hint: &quot;Mentálne postihnutie v kombinácií s iným postihnutím.&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        pokracovanieVPlneniPPV: {
            label: &quot;Pokračovanie v plnení povinného predprimárneho vzdelávania&quot;,
            hint: &quot;Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.&quot;
        },
        specialneVVP: {
            label: &quot;Má žiak špeciálne výchovno-vzdelávacie potreby?&quot;,
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        mentalnePostihnutie: {
            label: &quot;Má žiak mentálne postihnutie?&quot;,
            options: [
                {
                    label: &quot;Áno&quot;,
                    value: true
                },
                {
                    label: &quot;Nie&quot;,
                    value: false
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        }
    };

    const pageTextsDoplnujuceUdaje = {
        vzdelavanieVJazykuLabelMS: &quot; , &quot;'&quot; , &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot; , &quot;'&quot; , &quot;,
        vzdelavanieVJazykuLabelZS: &quot; , &quot;'&quot; , &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot; , &quot;'&quot; , &quot;,
        sectionHeaderDietaText: &quot; , &quot;'&quot; , &quot;Doplňujúce informácie o dieťati&quot; , &quot;'&quot; , &quot;,
        sectionHeaderZiakText: &quot; , &quot;'&quot; , &quot;Doplňujúce informácie o žiakovi&quot; , &quot;'&quot; , &quot;,
        sectionSubHeaderDietaText: &quot; , &quot;'&quot; , &quot;Vyplňte dodatočné informácie o potrebách vášho dieťaťa.&quot; , &quot;'&quot; , &quot;,
        sectionSubHeaderZiakText: &quot; , &quot;'&quot; , &quot;V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. &quot; , &quot;'&quot; , &quot;
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách vášho dieťaťa.
    

    

        Polia označené hviezdičkou sú povinné

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
            

    
    
        Požadovaná výchova
        (nepovinné)
    
    
    
    
                        
                        Etická 
                         
                        
                        Náboženská 
                         
    

        

        
            
            

    
    
        Vyberte typ
        (nepovinné)
    
    
    
    
                        
                        Rímskokatolícka 
                         
                        
                        Evanjelická 
                         
                        
                        Pravoslávna 
                         
                        
                        Reformovaná 
                         
                        
                        Gréckokatolícka 
                         
                        
                        Iná. Zadajte typ: 
                         
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v školskej jedálni
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Záujem o Školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Zdravotné znevýhodnenie dieťaťa
        (nepovinné)
    
    Dieťa so zdravotným postihnutím, zdravotne oslabené dieťa, dieťa s vývinovými poruchami alebo poruchou správania.
    
                        
                        Áno 
                         
            
            
                

    
        Popis znevýhodnenia
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Dieťa s nadaním
        (nepovinné)
    
    Dieťa, ktoré má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
                        Áno 
                         
            
            
                

    
        Popis nadania
        (nepovinné)
        
    
    
        warning
        
        
            0/500
        
    
    


            
        
                        
                        Nie 
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť?
        (nepovinné)
    
    Ak má žiak zdravotné obmedzenia, ktoré ovplyvňujú jeho schopnosť vykonávať určité činnosti alebo študovať v konkrétnom odbore, zvoľte &quot;Áno&quot;. V tom prípade je nutné priložiť aj lekárske potvrdenie alebo kópiu preukazu ŤZP. 
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
        

        
            

    
    
        Má žiak špeciálne výchovno-vzdelávacie potreby?
        (nepovinné)
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opatrenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        So zdravotným znevýhodnením 
                         Žiak so zdravotným postihnutím, chorý alebo zdravotne oslabený žiak, žiak s vývinovými poruchami alebo poruchou správania.  
                        
                        Zo sociálne znevýhodneného prostredia 
                         Žiak žijúci v prostredí, ktoré vzhľadom na sociálne, rodinné, ekonomické a kultúrne podmienky nedostatočne podnecuje rozvoj mentálnych, vôľových, emocionálnych vlastností žiaka, nepodporuje jeho socializáciu a neposkytuje mu dostatok primeraných podnetov pre rozvoj jeho osobnosti.  
                        
                        S nadaním 
                         Žiak, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        
            

    
    
        Má žiak mentálne postihnutie?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
            
            
                

    
    
        Vyberte
        (nepovinné)
    
    
    
    
                        
                        Mentálne postihnutie 
                         
                        
                        Viacnásobné postihnutie 
                         Mentálne postihnutie v kombinácií s iným postihnutím.  
    

            
        
                        
                        Nie 
                         
    

            
        

        

        

    
        Požadovaný dátum prijatia dieťaťa do materskej školy
        (nepovinné)
    
    
    
        
            

    
        Deň
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Mesiac
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        Rok
        (nepovinné)
    
    
    
        
        warning
        
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        «F Y»PoUtStŠtPiSoNe                                          Clear date    
    



    window[&quot; , &quot;'&quot; , &quot;pozadovanyDatumPrijatiaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        pozadovanyDatumPrijatiaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        pozadovanyDatumPrijatiaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        
            

    
        Poznámka:
        (nepovinné)
        
    
    
        warning
        
        
            4/500
        
    
    Môžete uviesť doplňujúce informácie, stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka. Ak má dieťa súrodenca na škole, uveďte do poznámky meno a priezvisko súrodenca a základnú školu ktorú navštevuje.


        

        
            
            

    
    
    
        
            
                
                    Pokračovanie v plnení povinného predprimárneho vzdelávania
                    (nepovinné)
                
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

        
    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        povinnePopis: &quot;Polia označené hviezdičkou sú povinné&quot;,
        radioLabel: &quot;Kam chcete podať prihlášku?&quot;,
        optionZS: {
            label: &quot;Na základnú školu&quot;,
            value: &quot;ZŠ&quot;
        },
        optionMS:
        {
            label: &quot;Na materskú školu&quot;,
            value: &quot;MŠ&quot;
        },
        &quot; , &quot;'&quot; , &quot;8060&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na materskú ani na základnú školu.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8061&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8062&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Na prihláške je uvedených $pocetSkol$ škol, čo je viac škôl ako je povolený počet $maxPocetSkol$.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8072&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na základnú školu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8073&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Podľa veku dieťaťa nie je možné podať prihlášku na materskú školu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PodanieDoMSNieJeDostupne&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Elektronické podanie prihlášky do materskej školy nie je dostupné&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PodanieDoZSNieJeDostupne&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Elektronické podanie prihlášky do základnej školy nie je dostupné&quot; , &quot;'&quot; , &quot;,

        vyberSkolyTitle: &quot;Vybrať školy&quot;,
        vyberSkolyTitleSS: &quot;Vybrať školy a odbory&quot;,
        vyberSkolyDescription: &quot;Pridajte všetky školy, uvedené na prihláške.&quot;,
        vyberSkolyDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške a následne ich kliknutím a potiahnutím zoraďte podľa poradia v akom sú uvedené. Poradie odboru určuje jeho preferenciu.&quot;,
        vyberSkolyDescriptionSSKolo2: &quot;Pridajte odbor uvedený na prihláške.&quot;,

        vyberSkolyPridaneTitleSS: &quot;Pridané školy a odbory&quot;,
        vyberSkolyPridane0TextSS: &quot;Nie je pridaná žiadna škola ani odbor.&quot;,
        vyberSkolyPridaneDescriptionSS: &quot;Pridajte všetky školy a odbory, uvedené na prihláške.&quot;,

        vyberSkolyPoradieDescription: &quot;Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.&quot;,

        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolHeader: &quot;Do prihlášky ste pridali maximálny počet škôl&quot;,
        warningMaxPocetSkolText2: &quot;Ak chcete pridať ďalšiu školu, najskôr jednu odstráňte.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;Ak chcete pridať ďalší odbor, najskôr jeden odstráňte.&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetSkolZodpovedaKriteriam: &quot; škôl zodpovedá vašim kritériám.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;&quot;,
        pocetOdborovVPrihlaskeText2: &quot;&quot;,
        pocetOdborovVPrihlaskeText5: &quot;&quot;,

        pocetOdborovText1: &quot;odbor zodpovedá vašim kritériám.&quot;,
        pocetOdborovText2: &quot;odbory zodpovedajú vašim kritériám.&quot;,
        pocetOdborovText5: &quot;odborov zodpovedá vašim kritériám.&quot;,

        jazykSelectMS: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        jazykSelectZS: {
            label: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]
        },

        zaujemOUvodnyAleboPripravnyRocnik: {
            label: &quot;Mám záujem o prípravný alebo úvodný ročník&quot;,
            showRequiredOrOptional: true,
            required: false,
        },

        szsZS: {
            label: &quot;Vyberte&quot;,
            showRequiredOrOptional: true,
            required: true,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
                    value: &quot;UVODNY_ROCNIK&quot;,
                },
            ]
        },

        notSzsZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },
        notSzsNadaneZS: {
            label: &quot;Záujem o úvodný ročník&quot;,
            hint: &quot;Dieťa so všeobecným intelektovým nadaním, ktoré dosiahlo 5 rokov alebo 4 roky a je u neho predpoklad zvládnutia prvého ročníka základnej školy.&quot;,
            showRequiredOrOptional: true,
            required: false,
        },


        dualneVzdelavanie: {
            label: &quot;Záujem o prípravu v systéme duálneho vzdelávania&quot;,
            hint: &quot;Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.&quot;,
            showRequiredOrOptional: false,
            required: false,

        },
        internat: {
            label: &quot;Záujem o školský internát&quot;,
            showRequiredOrOptional: false,
            required: false,
        },
        terminPrijimacejSkusky: {
            label: &quot;Termín prijímacej skúšky&quot;,
            showRequiredOrOptional: true,
            required: true,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }
            ]

        },
    }



    
        Vybrať školy
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        

        
            
                Pridané školy
            
            
                1
                školu ste pridali do prihlášky.
            
        

        
            Moja škola
            
                
                    Základná škola pre AT
                
                
                    EDUID 
                    910021625
                
                
                    Jalmová 266/19, 06534 Prešov
                
            
            
                

    
        Vzdelávanie dieťaťa žiadam poskytovať v jazyku
        (nepovinné)
    
    
    
        slovenský
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            
            
                
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                

                
                    

    
    
    
        
            
                
                    Záujem o úvodný ročník
                    (nepovinné)
                
                Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
            
            Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.
        
    

                
            
            
                
            
        

        
            
                add
                Pridať školu
            
        

        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Základná škola pre AT
                    
                    
                        Jalmová 266/19, 06534 Prešov
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca {osoba}
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;zastupca1DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1DatumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca1DatumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1InaAdresaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        zastupca1InaAdresaObec: {
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
        zastupca1InaAdresaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        zastupca1InaAdresaUlicaReq: {
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
        zastupca1InaAdresaSupisneCislo: {
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
        zastupca1InaAdresaOrientacneCislo: {
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
        zastupca1InaAdresaPSC: {
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
        zastupca1InaAdresaAdresa: {
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


            

            

            
                Osobné údaje zákonného zástupcu č. 2
                
                    

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                
                
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

                    

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;zastupca2DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2DatumNarodeniaDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        zastupca2DatumNarodeniaRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };



                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

                    
                    
                        
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;zastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2AdresaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        zastupca2AdresaObec: {
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
        zastupca2AdresaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        zastupca2AdresaUlicaReq: {
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
        zastupca2AdresaSupisneCislo: {
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
        zastupca2AdresaOrientacneCislo: {
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
        zastupca2AdresaPSC: {
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
        zastupca2AdresaAdresa: {
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


                    
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

    

    
        

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


    



    window[&quot; , &quot;'&quot; , &quot;adresaZariadeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZariadeniaKrajina: {
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Ak slovenskú adresu neviete zadať výberom hodnôt z číselníkov, vyberte možnosť „Slovenská republika – mimo číselník“ a vyplňte ju voľným textom.&quot; , &quot;'&quot; , &quot;,
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
        adresaZariadeniaObec: {
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
        adresaZariadeniaUlica: {
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,
            required: false,
            customOptionalText: &quot; , &quot;'&quot; , &quot;(ak existuje – potrebné pre doručovanie rozhodnutia)&quot; , &quot;'&quot; , &quot;,
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
        adresaZariadeniaUlicaReq: {
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
        adresaZariadeniaSupisneCislo: {
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
        adresaZariadeniaOrientacneCislo: {
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
        adresaZariadeniaPSC: {
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
        adresaZariadeniaAdresa: {
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


                        
        
    

                    
                    
                    
                    
                    
                        

    const pageTextsPrilohy = {
        &quot; , &quot;'&quot; , &quot;8106&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8105&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne nahratý.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8104&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8113&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8108&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne vymazaný.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8109&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8110&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli pridané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ziadnePovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nie sú potrebné žiadne povinné prílohy.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahrajteYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Prosím, nahrajte všetky povinné prílohy.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahraneYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrané prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyChybajuceYellow&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Chýbajúce prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PrilohyNahraneSuborySuccess&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;VsetkyPotrebnePrilohyNahrane&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli nahrané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;Nahrane0Priloh&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ChybajuPovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Chýbajú povinné prílohy&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NahrajteNasledujucePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrajte nasledujúce prílohy:&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;kodOdboru&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Kód odboru&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;jazyk&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Jazyk&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;olympiadaSutaz&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Olympiáda / súťaž&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;skolskyRok&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Školský rok&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;druhSportu&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Druh športu&quot; , &quot;'&quot; , &quot;,
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect: {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        },

        modalPridatInuPrilohuNazovPrilohy: {
            label: &quot;Typ prílohy&quot;,
            required: true,
            type: &quot;input&quot;,
            name: &quot; , &quot;'&quot; , &quot;inyTypPrilohy&quot; , &quot;'&quot; , &quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;
                }

            ]
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
    

    
        
            Nahrané iné súbory
        
        
        
    

    

    
        add
        Pridať inú prílohu
    

    
    

    
    

    
        
            Zvoľte, ako chcete nahrať súbor:
            Súbory
            Knižnica fotografií
            Fotoaparát
        
        Cancel
    

    

    

    


                    
                    
                        

    const controlSettingsOU = {
        aktualnyRokValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Zadajte dátum z aktuálneho roka.&quot; , &quot;'&quot; , &quot;,
        rokVBuducnostiValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Zadaný dátum musí by aktuálny dátum alebo starší ako aktuálny dátum.&quot; , &quot;'&quot; , &quot;,
        tvarDatumuValidationErrorMessage: &quot; , &quot;'&quot; , &quot;Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.08.2025).&quot; , &quot;'&quot; , &quot;,
        povinneErrorMessage: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;,
        datumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
             validators: []
        },
        OUPoznamkaText: {
            label: &quot;Poznámka školy:&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[\s\S]{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },

        &quot; , &quot;'&quot; , &quot;8084&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.&quot; , &quot;'&quot; , &quot;,

    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    Polia označené hviezdičkou sú povinné

    
        Dátum prihlášky

        

    
        
        (nepovinné)
    
    
    
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

        
        
             
            
            
                
                
                    
                        calendar_month
                    
                
            
        
        
    



    window[&quot; , &quot;'&quot; , &quot;datumPodaniaPrihlaskyControlSettings&quot; , &quot;'&quot; , &quot;] = {
        datumPodaniaPrihlaskyDen: {
            label: &quot; , &quot;'&quot; , &quot;Deň&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;DD&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyMesiac: {
            label: &quot; , &quot;'&quot; , &quot;Mesiac&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                maxLength: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;MM&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|1[0-2])$/,
                }
            ],
            required: false
        },
        datumPodaniaPrihlaskyRok: {
            label: &quot; , &quot;'&quot; , &quot;Rok&quot; , &quot;'&quot; , &quot;,
            showRequiredOrOptional: false,
            attributes: {
                minLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                maxLength: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;,
                placeholder: &quot; , &quot;'&quot; , &quot;YYYY&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^([0-9]{4})$/,
                }
            ],
            required: false
        },
    };




        Poznámka k prihláške

        
            

    
        
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        dietaHeaderMS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderZS: &quot;Osobné údaje dieťaťa&quot;,
        dietaHeaderSS: &quot;Osobné údaje žiaka&quot;,
        skolyHeaderMS: &quot;Materská škola&quot;,
        skolyHeaderZS: &quot;Základná škola&quot;,
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaInfoHeaderMS: &quot;Materská škola č.&quot;,
        skolaInfoHeaderZS: &quot;Základná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;Poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;Celodennú výchovu a vzdelávanie&quot;,
        spravaDovod: &quot;&quot;,
        spravaPrilozeneDokumenty: &quot;&quot;,
        nastupPotvrdeny: &quot;&quot;,
        nenastupi: &quot;&quot;,
        identifikator: &quot;&quot;,
        dietati: &quot; , &quot;'&quot; , &quot;dieťati&quot; , &quot;'&quot; , &quot;,
        dietata: &quot; , &quot;'&quot; , &quot;dieťaťa&quot; , &quot;'&quot; , &quot;,
        ziakovi: &quot; , &quot;'&quot; , &quot;žiakovi&quot; , &quot;'&quot; , &quot;,
        ziaka: &quot; , &quot;'&quot; , &quot;žiaka&quot; , &quot;'&quot; , &quot;,
        zdravotneZnevyhodnenie: &quot;So zdravotným znevýhodnením&quot;,
        socialneZnevyhodnenie: &quot;Zo sociálne znevýhodneného prostredia&quot;,
        nadanie: &quot;S nadaním&quot;,
        mentalne: &quot;Mentálne zdravotné postihnutie&quot;,
        mentalneSInym: &quot;Viacnásobné postihnutie&quot;,
        kolo: &quot;{kolo}. kolo&quot;,
        skolskyRok: &quot;Školský rok:&quot;,
        uroven: &quot;úroveň&quot;,
        prichodZoZSNaSVK: &quot;Zo ZŠ na Slovensku&quot;,
        prichodZoZahranicia: &quot;Zo školy v zahraničí&quot;,
        zPraxe: &quot;Z praxe&quot;,
        ine: &quot;Iné&quot;,
        eduidSkoly: &quot;EDUID školy&quot;,
        nazovStrednejSkoly: &quot;Názov strednej školy&quot;,
        kodOdboru: &quot;Kód odboru&quot;,
        nazovOdboru: &quot;Názov odboru&quot;,
        typOdboru: &quot;Typ odboru&quot;,
        terminPrijimacejSkusky: &quot;Termín prijímacej skúšky&quot;,
        vyucovaciJazykOdboru: &quot;Vyučovací jazyk odboru&quot;,
        zaujemODualneVzdelavanie: &quot;Záujem o duálne vzdelávanie&quot;,
        zaujemOSkolskyInternat: &quot;Záujem o školský internát&quot;,
        talentovy: &quot;Talentový&quot;,
        netalentovy: &quot;Netalentový&quot;,
        zastupcoviaHeader: &quot;Osobné údaje zákonných zástupcov žiaka&quot;,
        zastupcoviaHeaderMSZS: &quot;Osobné údaje zákonných zástupcov dieťaťa&quot;,
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        sidloMS: &quot;Sídlo materskej školy&quot;,
        sidloZS: &quot;Sídlo základnej školy&quot;,
        vzdelavanieVJazykuLabelZS: &quot;Vzdelávanie dieťaťa žiadam poskytovať v jazyku&quot;,
        zaujemOPripravnyRocnik: &quot;Záujem o prípravný ročník&quot;,
        zaujemOUvodnyRocnik: &quot;Záujem o úvodný ročník&quot;,
        predprimarneVzdelavanie: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot; target=&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }

    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        
            
                Všeobecné informácie
            
            Upraviť
        
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Poznámka školy
                -
            
            
                Kolo prijímacieho konania
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
            Upraviť
        
        
            
                Meno
                -
            
            
                Priezvisko
                -
            
            
                Rodné priezvisko
                -
            
            
                Rodné číslo
                -
            
            
                Dátum narodenia
                -
            
            
                Pohlavie
                -
            
            
                Miesto narodenia
                -
            
            
                Národnosť
                -
            
            
                Štátna príslušnosť
                -
            
            
                Materinský jazyk
                -
            
            
                Iný materinský jazyk
                -
            
            
                Adresa trvalého pobytu
                -
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu.
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v školskej jedálni
                        
                    
                    
                        Záujem o školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa
                        

                    
                    
                        Dieťa s nadaním
                        
                    
                    
                        Požadovaný dátum prijatia dieťaťa do materskej školy
                        -
                    
                    
                        Popis znevýhodnenia
                        
                    
                    
                        Popis nadania
                        
                    
                    
                        Poznámka
                        
                    
                    
                        Pokračovanie v plnení povinného predprimárneho vzdelávania
                        -
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Špeciálne výchovno-vzdelávacie potreby
                        -
                    
                    
                        Mentálne postihnutie
                        -
                    
                    
                        
                        
                    
                    
                        Poznámka
                        -
                    
                
            
        
    

    
        
            
                Stredná škola
            
            Upraviť
        
        
        
    

    
        
            
            
            Upraviť
        
        
            
                Osobné údaje zákonného zástupcu č. 1
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Súhlas s komunikáciou výhradne so zákonným zástupcom č. 1
                        -
                    
                
                
                Osobné údaje zákonného zástupcu č. 2
                
                    
                        Meno
                        -
                    
                    
                        Priezvisko
                        -
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Dátum narodenia
                        -
                    
                    
                        Korešpondenčná adresa
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                    
                        Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca dieťaťa
                        -
                    
                    
                        Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                        -
                    
                
                Druhý zákonný zástupca nie je známy.
            
            
                
                    
                        Názov zariadenia
                        -
                    
                    
                        IČO zariadenia
                        -
                    
                    
                        Adresa zariadenia
                        -
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        E-mail
                        -
                    
                    
                        Telefónne číslo
                        -
                    
                
            
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                -
            
            
                EDUID základnej školy
                -
            
            
                Názov základnej školy
                -
            
            
                Ročník
                -
            
            
                Trieda
                -
            
            
                Rok školskej dochádzky
                -
            
            
                Vyučovací jazyk v základnej škole
                -
            
        
    

    
        
            
                Výsledky vzdelávania na základnej škole
            
            Upraviť
        
        
        
    

    
        
            
                Súťaže
            
            Upraviť
        
        
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            
            
        
    

    
        
            
                
            
            
                Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                

    
    
    
        
            
                
                    
                    (nepovinné)
                
                
            
            
        
    

            
        
    

    





EXPORT PDF

                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                &quot;))]</value>
      <webElementGuid>be5b44d1-7c19-43ac-a53e-c8e8ceed6991</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
