<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Odhlsi_govuk-wizard-steps</name>
   <tag></tag>
   <elementGuidId>cf3f4899-513d-42d6-9dd6-01a3a7921e13</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.govuk-wizard-steps</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//fieldset[1]/div[3]/div[2]/div[6]/div[1]/div[3]/div[12]</value>
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
      <webElementGuid>125973c8-0791-4567-bc11-8c762198adfd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-wizard-steps</value>
      <webElementGuid>db136b8c-a904-4d90-a70d-55626bc3d8a7</webElementGuid>
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
    

                    
                    
                        
                            
                                Krok5/10
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje žiaka
                        
                    
                        
                            2.
                            Doplňujúce údaje o žiakovi
                        
                    
                        
                            3.
                            Vybrať školy a odbory
                        
                    
                        
                            4.
                            Zákonný zástupca žiaka
                        
                    
                        
                            5.
                            Informácie o základnej škole
                        
                    
                        
                            6.
                            Výsledky vzdelávania
                        
                    
                        
                            7.
                            Súťaže
                        
                    
                        
                            8.
                            Pridať prílohy
                        
                    
                        
                            9.
                            Ostatné údaje
                        
                    
                        
                            10.
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
                        Samuel
                    
                    
                        Priezvisko
                        Horváth
                    
                    
                        Rodné priezvisko
                        Horváth
                    
                    
                        Rodné číslo
                        090716/3939
                    
                    
                        Dátum narodenia
                        16.07.2009
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Nitra
                    
                    
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
                        Štúrova 1435, Nitra, Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                        -
                    
                    
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
        
    
Slovenská republika

                    
                    
                        

    
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
    
    
    
    
                        
                        Štúrova 1435, Nitra, Slovenská republika 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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
            hint: &quot;Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v jedálni&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
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
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
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



    Doplňujúce informácie o žiakovi

    
        V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. 
    

    

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
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v jedálni
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
    
    Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
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
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
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
        
        
            0/500
        
    
    Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.


        

        
            
            

    
    
    
        
            
                
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

        szsZS: {
            label: &quot;Záujem o&quot;,
            showRequiredOrOptional: true,
            required: false,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
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



    
        Vybrať školy a odbory
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        Polia označené hviezdičkou sú povinné

        
            
                Pridané školy
            
            
                
                
            
        

        Polia označené hviezdičkou sú povinné
            Moja škola
            
                
                    Gymnázium Metodova
                
                
                    EDUID 
                    910020859
                
                
                    Metodova 2, 82108, Bratislava
                
            

            
            
            
                
                    gymnázium
                    Netalentový odbor
                
                
                    Kód odboru: 7902J00
                     • 
                    Vyučovací jazyk: slovenský
                
            

            
                
                    

    
    
    
        
            
                
                    Záujem o prípravu v systéme duálneho vzdelávania
                    (nepovinné)
                
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                
                
                    

    
    
    
        
            
                
                    Záujem o školský internát
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                

                Odstrániť z prihlášky

            

        
        

        

        
            
                add
                Pridať odbor mojej školy
            
            
                add
                
                Pridať odbor
            
        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Gymnázium Metodova
                    
                    
                        
                            gymnázium
                             • 
                            1. termín (1.kolo)
                        
                        Netalentový odbor
                    
                    
                        Metodova 2, 82108, Bratislava
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca dieťaťa
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        Prihlášku podáva:
        (nepovinné)
    
    
    
    
                        
                        Zákonný zástupca 
                         
                        
                        Zástupca zariadenia 
                         
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
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
            
        
    

                
                
                    

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                

    
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



                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt rodiča z RFO  
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

            
            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window['zastupca1InaAdresaControlSettings'] = {
        zastupca1InaAdresaKrajina: {
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
                
                    

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                    
                        

    
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
            
        
    

                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                    

    
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



                    
                        

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

                    
                    
                        
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window['zastupca2AdresaControlSettings'] = {
        zastupca2AdresaKrajina: {
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


                    
                
                        
                        Druhý zákonný zástupca nie je známy 
                         Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.  
    

                
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                

    
    
    
        
            
                
                    Zákonní zástupcovia žiadajú, a dokladujú to vyhlásením oboch zákonných zástupcov, aby sa vo veciach súvisiacich s prijímacím konaním dieťaťa komunikovalo výhradne so zákonným zástupcom č. 1.
                    (nepovinné)
                
                
            
            
        
    

            
                        
                        nie 
                         
    

            
            
            
                
                    

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window['adresaZariadeniaControlSettings'] = {
        adresaZariadeniaKrajina: {
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


                        
        
    

                    
                    
                        

    const controlSettingsNajstSkolu = {
        loading: 'Načítava sa'
    };

    const controlSettingsInfoOZS = {
        prichodZiakaRadio : {
            label: &quot;Príchod žiaka:&quot;,
            options: [
                {
                    label: &quot;Zo ZŠ na Slovensku&quot;,
                    value: '1'
                },
                {
                    label: &quot;Zo školy v zahraničí&quot;,
                    value: '2'
                },
                {
                    label: &quot;Z praxe&quot;,
                    value: '3',
                    id: 'ddd',
                    class: 'aaa'
                },
                {
                    label: &quot;Iné&quot;,
                    value: '4'
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
        rocnikSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        eduidSkolyInput: {
            label: &quot;EDUID základnej školy&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        rocnikSKSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        triedaInput: {
            label: &quot;Trieda&quot;,
            hint: &quot;Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9Á-Žá-žA-Za-z\,\.\'\-\s]{1,10}$/,
                    message: &quot;Môžete zadať maximálne 10 znakov.&quot;,
                    regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySKSelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleSKAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        rocnikZPraxeSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        rocnikIneSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte dodatočné informácie o základnej škole.
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                
            
            
                EDUID základnej školy
                
            
            
                Názov základnej školy
                
            
            
                Ročník
                
            
            
                Trieda
                
            
            
                Rok školskej dochádzky
                
            
            
                Vyučovací jazyk v základnej škole
                
            
        
    

    
        
            

    
    
        Príchod žiaka:
        (nepovinné)
    
    
    
    
                        
                        Zo ZŠ na Slovensku 
                         

            
                Názov základnej školy
                Uveďte školu, ktorú žiak navštevuje.
                
                    
                    warning
                
                Toto pole je povinné.
                
            

            
                

    
        EDUID základnej školy
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Trieda
        (nepovinné)
    
    Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
                        
                        Zo školy v zahraničí 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Z praxe 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
                        
                        Iné 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
    

        

        

        

        

        
    

    
        
            
                
                    
                        Upraviť
                        Informácie o základnej škole
                    
                    
                        
                            close
                        
                    
                
                
                    Polia označené hviezdičkou sú povinné
                    
                
                
                    Zrušiť
                    Uložiť
                
            
        
        
    

    

                    
                    
                        

    const pageTextsVysledkyVzdelavania = {
        polrok: &quot;polrok&quot;,
        rocnik: &quot;ročník&quot;,
        vyberteHodnotenie: &quot;Vyberte hodnotenie&quot;,
        zmeniliSteUdajePanelTitle: &quot;Pre overenie správnosti údajov bude požadované:&quot;,
        zmeniliSteUdajePanelText: &quot;Zmenili ste údaje na jednom predmete / viacerých predmetoch, preto bude nutné priložiť k prihláške:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;&quot;,
        neuviedliSteUdajePanelText: &quot;&quot;,
        slovneHodnoteniePanelDescription: &quot;V prípade použitia možnosti slovné hodnotenie sa k prihláške doloží kópia vysvedčenia za daný ročník.&quot;,
        modalDescription: &quot;Vyberte predmet a hodnotenie pre %rocnik%. ročník, podľa vysvedčenia.&quot;,
        overenuKopiu: &quot;overenú kópiu vysvedčenia pre &quot;
    };

    const controlSettingsVysledkyVzdelavania = {
        hodnotenieTable: {
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        inaZnamkovaSchema: {
            label: &quot;&quot;,
            hint: &quot;&quot;,
            required: false
        },
        modalPredmet: {
            label: &quot;Predmet&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        modalJazyk: {
            label: &quot;Jazyk&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        modalHodnotenie: {
            label: &quot;Hodnotenie&quot;,
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



    Výsledky vzdelávania na základnej škole

    
        Doplňte výsledky vzdelávania z posledných 4 ročníkov základnej školy podľa vysvedčenia. V prípade, že žiak opakoval ročník, uveďte známky z posledného ročníka.
    

    
        
        
    

    
        info
        
            Dôležité
            Ak boli k prihláške priložené vysvedčenia s inou známkovacou škálou, než aká sa používa na slovenských školách, pred ich zápisom ich preveďte na štandardné známky podľa tohto dokumentu.
        
        Stiahnuť dokument
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
        

        
    
    

    
        
            
                
                    Pridať predmet
                
                
                    
                        close
                    
                
                
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    

                    
                    
                        

    const controlSettingsSutaze = {
         modalNazovSutazeText: {
            label: &quot;Názov súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Zadajte názov súťaže.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9%\s\S]{1,255}$/,
                    message: &quot;Zadajte názov súťaže.&quot;,
                    regexError: &quot;Zadajte názov súťaže.&quot;
                }
            ],
            required: true

        },
        modalDruhSutazeSelect: {
            label: &quot;Druh súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        modalDruhSportuSelect: {
            label: &quot;Druh športu&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        modalUrovenSutazeSelect: {
            label: &quot;Úroveň súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
        },
        modalTypUmiestneniaRadio : {
            label: &quot;Typ umiestnenia&quot;,
            options: [
                // {
                //     label: &quot;1. miesto&quot;,
                //     value: '1'
                // },
                // {
                //     label: &quot;2. miesto&quot;,
                //     value: '2'
                // },
                // {
                //     label: &quot;3. miesto&quot;,
                //     value: '3'
                // },
                // {
                //     label: &quot;Bez umiestnenia&quot;,
                //     value: '99'
                // },
                {
                    label: &quot;Iné&quot;,
                    value: '5'
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

        modalTypUmiestneniaIneText: {
            label: &quot;Zadajte umiestnenie&quot;,
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
        modalSkolskyRokSelect: {
            label: &quot;Školský rok&quot;,
            hint: &quot;V ktorom sa žiak zúčastnil súťaže.&quot;,
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

    const pageTextsSutaze = {
        vzdelavanieVJazykuLabelMS: '',
        skolskyRok: 'Školský rok',
        uroven: 'úroveň',
    };



    Súťaže

    
        Uveďte súťaže, v ktorých žiak dosiahol výborné výsledky alebo umiestnenia.
    

    
        
            
                
                    Súťaž
                    Umiestnenie
                    Akcia
                
            
            
                
                    
                        
                            
                                add
                            
                            
                                Pridať súťaž
                            
                        
                    
                
            
        
    

    

    
        
            
                
                    
                        Pridať súťaž
                    
                    
                        Pokiaľ pridávate súťaž, bude povinné priložiť diplom o úspešnom absolvovaní.
                    
                
                
                    
                        close
                    
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
    
        
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    


                    
                    
                        

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
                    functionOrRegex: /^$|^.{1,500}$/,
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
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
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
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href='https://www.slovensko.sk/'>slovensko.sk&lt;/a>.&lt;/div>&quot;,
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
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v jedálni
                        
                    
                    
                        Záujem o Školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        
                    
                    
                        Popis znevýhodnenia / nadania
                        
                    
                    
                        Poznámka
                        
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Upravené podmienky prijímacieho konania (Špeciálne výchovno-vzdelávacie potreby)
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
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa zariadenia
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
      <webElementGuid>7408058a-41ae-4825-b775-abb57b0f3760</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ziadost&quot;)/div[@class=&quot;menu-layout&quot;]/div[@class=&quot;privatna-zona-content&quot;]/div[@class=&quot;govuk-form-group&quot;]/div[@class=&quot;govuk-wizard-steps&quot;]</value>
      <webElementGuid>adc93d69-979c-4dbb-8ae7-8949088a0174</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='ziadost']/div/div/div/div</value>
      <webElementGuid>97b0550a-fc81-4bc7-b537-b3058470058a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Odhlásiť'])[2]/following::div[5]</value>
      <webElementGuid>7adf9f59-7be1-4fe0-a8fe-c8925db4f27f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Oznámenia'])[2]/following::div[5]</value>
      <webElementGuid>a65d8858-e660-449a-8b66-0e5eb1dbf595</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body/div[2]/div/div/div/div</value>
      <webElementGuid>8060b832-2fd2-4dd2-af0e-67dc4a2de2b2</webElementGuid>
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
    

                    
                    
                        
                            
                                Krok5/10
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje žiaka
                        
                    
                        
                            2.
                            Doplňujúce údaje o žiakovi
                        
                    
                        
                            3.
                            Vybrať školy a odbory
                        
                    
                        
                            4.
                            Zákonný zástupca žiaka
                        
                    
                        
                            5.
                            Informácie o základnej škole
                        
                    
                        
                            6.
                            Výsledky vzdelávania
                        
                    
                        
                            7.
                            Súťaže
                        
                    
                        
                            8.
                            Pridať prílohy
                        
                    
                        
                            9.
                            Ostatné údaje
                        
                    
                        
                            10.
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
                        Samuel
                    
                    
                        Priezvisko
                        Horváth
                    
                    
                        Rodné priezvisko
                        Horváth
                    
                    
                        Rodné číslo
                        090716/3939
                    
                    
                        Dátum narodenia
                        16.07.2009
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Nitra
                    
                    
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
                        Štúrova 1435, Nitra, Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                        -
                    
                    
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
        
    
Slovenská republika

                    
                    
                        

    
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
    
    
    
    
                        
                        Štúrova 1435, Nitra, Slovenská republika 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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
            hint: &quot;Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v jedálni&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
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
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
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



    Doplňujúce informácie o žiakovi

    
        V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. 
    

    

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
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v jedálni
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
    
    Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
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
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
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
        
        
            0/500
        
    
    Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.


        

        
            
            

    
    
    
        
            
                
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

        szsZS: {
            label: &quot;Záujem o&quot;,
            showRequiredOrOptional: true,
            required: false,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
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



    
        Vybrať školy a odbory
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        Polia označené hviezdičkou sú povinné

        
            
                Pridané školy
            
            
                
                
            
        

        Polia označené hviezdičkou sú povinné
            Moja škola
            
                
                    Gymnázium Metodova
                
                
                    EDUID 
                    910020859
                
                
                    Metodova 2, 82108, Bratislava
                
            

            
            
            
                
                    gymnázium
                    Netalentový odbor
                
                
                    Kód odboru: 7902J00
                     • 
                    Vyučovací jazyk: slovenský
                
            

            
                
                    

    
    
    
        
            
                
                    Záujem o prípravu v systéme duálneho vzdelávania
                    (nepovinné)
                
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                
                
                    

    
    
    
        
            
                
                    Záujem o školský internát
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                

                Odstrániť z prihlášky

            

        
        

        

        
            
                add
                Pridať odbor mojej školy
            
            
                add
                
                Pridať odbor
            
        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Gymnázium Metodova
                    
                    
                        
                            gymnázium
                             • 
                            1. termín (1.kolo)
                        
                        Netalentový odbor
                    
                    
                        Metodova 2, 82108, Bratislava
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca dieťaťa
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        Prihlášku podáva:
        (nepovinné)
    
    
    
    
                        
                        Zákonný zástupca 
                         
                        
                        Zástupca zariadenia 
                         
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
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
            
        
    

                
                
                    

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                

    
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



                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt rodiča z RFO  
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

            
            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1InaAdresaKrajina: {
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
                
                    

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                    
                        

    
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
            
        
    

                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                    

    
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



                    
                        

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

                    
                    
                        
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;zastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2AdresaKrajina: {
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


                    
                
                        
                        Druhý zákonný zástupca nie je známy 
                         Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.  
    

                
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                

    
    
    
        
            
                
                    Zákonní zástupcovia žiadajú, a dokladujú to vyhlásením oboch zákonných zástupcov, aby sa vo veciach súvisiacich s prijímacím konaním dieťaťa komunikovalo výhradne so zákonným zástupcom č. 1.
                    (nepovinné)
                
                
            
            
        
    

            
                        
                        nie 
                         
    

            
            
            
                
                    

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;adresaZariadeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZariadeniaKrajina: {
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


                        
        
    

                    
                    
                        

    const controlSettingsNajstSkolu = {
        loading: &quot; , &quot;'&quot; , &quot;Načítava sa&quot; , &quot;'&quot; , &quot;
    };

    const controlSettingsInfoOZS = {
        prichodZiakaRadio : {
            label: &quot;Príchod žiaka:&quot;,
            options: [
                {
                    label: &quot;Zo ZŠ na Slovensku&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Zo školy v zahraničí&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Z praxe&quot;,
                    value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;,
                    id: &quot; , &quot;'&quot; , &quot;ddd&quot; , &quot;'&quot; , &quot;,
                    class: &quot; , &quot;'&quot; , &quot;aaa&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iné&quot;,
                    value: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;
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
        rocnikSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        eduidSkolyInput: {
            label: &quot;EDUID základnej školy&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikSKSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        triedaInput: {
            label: &quot;Trieda&quot;,
            hint: &quot;Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9Á-Žá-žA-Za-z\,\.\&quot; , &quot;'&quot; , &quot;\-\s]{1,10}$/,
                    message: &quot;Môžete zadať maximálne 10 znakov.&quot;,
                    regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySKSelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleSKAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikZPraxeSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikIneSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte dodatočné informácie o základnej škole.
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                
            
            
                EDUID základnej školy
                
            
            
                Názov základnej školy
                
            
            
                Ročník
                
            
            
                Trieda
                
            
            
                Rok školskej dochádzky
                
            
            
                Vyučovací jazyk v základnej škole
                
            
        
    

    
        
            

    
    
        Príchod žiaka:
        (nepovinné)
    
    
    
    
                        
                        Zo ZŠ na Slovensku 
                         

            
                Názov základnej školy
                Uveďte školu, ktorú žiak navštevuje.
                
                    
                    warning
                
                Toto pole je povinné.
                
            

            
                

    
        EDUID základnej školy
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Trieda
        (nepovinné)
    
    Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
                        
                        Zo školy v zahraničí 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Z praxe 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
                        
                        Iné 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
    

        

        

        

        

        
    

    
        
            
                
                    
                        Upraviť
                        Informácie o základnej škole
                    
                    
                        
                            close
                        
                    
                
                
                    Polia označené hviezdičkou sú povinné
                    
                
                
                    Zrušiť
                    Uložiť
                
            
        
        
    

    

                    
                    
                        

    const pageTextsVysledkyVzdelavania = {
        polrok: &quot;polrok&quot;,
        rocnik: &quot;ročník&quot;,
        vyberteHodnotenie: &quot;Vyberte hodnotenie&quot;,
        zmeniliSteUdajePanelTitle: &quot;Pre overenie správnosti údajov bude požadované:&quot;,
        zmeniliSteUdajePanelText: &quot;Zmenili ste údaje na jednom predmete / viacerých predmetoch, preto bude nutné priložiť k prihláške:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;&quot;,
        neuviedliSteUdajePanelText: &quot;&quot;,
        slovneHodnoteniePanelDescription: &quot;V prípade použitia možnosti slovné hodnotenie sa k prihláške doloží kópia vysvedčenia za daný ročník.&quot;,
        modalDescription: &quot;Vyberte predmet a hodnotenie pre %rocnik%. ročník, podľa vysvedčenia.&quot;,
        overenuKopiu: &quot;overenú kópiu vysvedčenia pre &quot;
    };

    const controlSettingsVysledkyVzdelavania = {
        hodnotenieTable: {
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        inaZnamkovaSchema: {
            label: &quot;&quot;,
            hint: &quot;&quot;,
            required: false
        },
        modalPredmet: {
            label: &quot;Predmet&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalJazyk: {
            label: &quot;Jazyk&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalHodnotenie: {
            label: &quot;Hodnotenie&quot;,
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



    Výsledky vzdelávania na základnej škole

    
        Doplňte výsledky vzdelávania z posledných 4 ročníkov základnej školy podľa vysvedčenia. V prípade, že žiak opakoval ročník, uveďte známky z posledného ročníka.
    

    
        
        
    

    
        info
        
            Dôležité
            Ak boli k prihláške priložené vysvedčenia s inou známkovacou škálou, než aká sa používa na slovenských školách, pred ich zápisom ich preveďte na štandardné známky podľa tohto dokumentu.
        
        Stiahnuť dokument
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
        

        
    
    

    
        
            
                
                    Pridať predmet
                
                
                    
                        close
                    
                
                
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    

                    
                    
                        

    const controlSettingsSutaze = {
         modalNazovSutazeText: {
            label: &quot;Názov súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Zadajte názov súťaže.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9%\s\S]{1,255}$/,
                    message: &quot;Zadajte názov súťaže.&quot;,
                    regexError: &quot;Zadajte názov súťaže.&quot;
                }
            ],
            required: true

        },
        modalDruhSutazeSelect: {
            label: &quot;Druh súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalDruhSportuSelect: {
            label: &quot;Druh športu&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalUrovenSutazeSelect: {
            label: &quot;Úroveň súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalTypUmiestneniaRadio : {
            label: &quot;Typ umiestnenia&quot;,
            options: [
                // {
                //     label: &quot;1. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;2. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;3. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;Bez umiestnenia&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;99&quot; , &quot;'&quot; , &quot;
                // },
                {
                    label: &quot;Iné&quot;,
                    value: &quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;
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

        modalTypUmiestneniaIneText: {
            label: &quot;Zadajte umiestnenie&quot;,
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
        modalSkolskyRokSelect: {
            label: &quot;Školský rok&quot;,
            hint: &quot;V ktorom sa žiak zúčastnil súťaže.&quot;,
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

    const pageTextsSutaze = {
        vzdelavanieVJazykuLabelMS: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
        skolskyRok: &quot; , &quot;'&quot; , &quot;Školský rok&quot; , &quot;'&quot; , &quot;,
        uroven: &quot; , &quot;'&quot; , &quot;úroveň&quot; , &quot;'&quot; , &quot;,
    };



    Súťaže

    
        Uveďte súťaže, v ktorých žiak dosiahol výborné výsledky alebo umiestnenia.
    

    
        
            
                
                    Súťaž
                    Umiestnenie
                    Akcia
                
            
            
                
                    
                        
                            
                                add
                            
                            
                                Pridať súťaž
                            
                        
                    
                
            
        
    

    

    
        
            
                
                    
                        Pridať súťaž
                    
                    
                        Pokiaľ pridávate súťaž, bude povinné priložiť diplom o úspešnom absolvovaní.
                    
                
                
                    
                        close
                    
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
    
        
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    


                    
                    
                        

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
                    functionOrRegex: /^$|^.{1,500}$/,
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
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
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
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
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
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v jedálni
                        
                    
                    
                        Záujem o Školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        
                    
                    
                        Popis znevýhodnenia / nadania
                        
                    
                    
                        Poznámka
                        
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Upravené podmienky prijímacieho konania (Špeciálne výchovno-vzdelávacie potreby)
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
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa zariadenia
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
    

                    
                    
                        
                            
                                Krok5/10
                            
                            
                                arrow_forward_ios
                                
                                    
                        
                            1.
                            Osobné údaje žiaka
                        
                    
                        
                            2.
                            Doplňujúce údaje o žiakovi
                        
                    
                        
                            3.
                            Vybrať školy a odbory
                        
                    
                        
                            4.
                            Zákonný zástupca žiaka
                        
                    
                        
                            5.
                            Informácie o základnej škole
                        
                    
                        
                            6.
                            Výsledky vzdelávania
                        
                    
                        
                            7.
                            Súťaže
                        
                    
                        
                            8.
                            Pridať prílohy
                        
                    
                        
                            9.
                            Ostatné údaje
                        
                    
                        
                            10.
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
                        Samuel
                    
                    
                        Priezvisko
                        Horváth
                    
                    
                        Rodné priezvisko
                        Horváth
                    
                    
                        Rodné číslo
                        090716/3939
                    
                    
                        Dátum narodenia
                        16.07.2009
                    
                    
                        Pohlavie
                        mužské
                    
                    
                        Miesto narodenia
                        Nitra
                    
                    
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
                        Štúrova 1435, Nitra, Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                        -
                    
                    
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
        
    
Slovenská republika

                    
                    
                        

    
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
    
    
    
    
                        
                        Štúrova 1435, Nitra, Slovenská republika 
                         
                        
                        Iná adresa trvalého pobytu 
                         
    

                    
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava
                        
                            

    
    
        Vyberte adresu
        (nepovinné)
    
    
    
    
                        
                        Zhodná s adresou trvalého pobytu dieťaťa 
                         
                        
                        Iná adresa 
                         
    

                        
                        
                            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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
            hint: &quot;Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        zsDPDStravovanieRadio : {
            label: &quot;Záujem o stravovanie v jedálni&quot;,
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
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Zadajte popis znevýhodnenia / nadania vášho dieťaťa.&quot;
                }
            ]

        },
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.&quot;,
            required: false,
            attributes: {
                maxLength: 500
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^.{1,500}$/,
                    message: &quot;Môžete zadať maximálne 500 znakov.&quot;
                }
            ]
        },
        pozadovanyDatumPrijatia: {
            label: &quot;Požadovaný dátum prijatia dieťaťa do materskej školy&quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            datumNarodeniaErrorNeplatny: &quot;Dátum narodenia nie je platný dátum.&quot;,
            required: true,
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
            hint: &quot;Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). &quot;,
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



    Doplňujúce informácie o žiakovi

    
        V tejto časti môžete uviesť informácie, ktoré môžu ovplyvniť priebeh vzdelávania žiaka - napríklad zdravotné obmedzenia alebo špeciálne výchovno-vzdelávacie potreby (ŠVVP). Ak žiak spĺňa niektorú z podmienok, bude potrebné priložiť potvrdenie od odborníka. 
    

    

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
            
        
    

        
    

        

        

        
            

    
    
        Záujem o stravovanie v jedálni
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
    
    Dieťa, ktorý má nadpriemerné schopnosti v intelektovej oblasti, v oblasti umenia alebo športu alebo v týchto oblastiach dosahuje v porovnaní s rovesníkmi mimoriadne výsledky alebo mimoriadne výkony.
    
                        
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
    
    Podľa odporúčania zo zariadení poradenstva a prevencie, ak žiak potrebuje podporné opaterenie vo vzdelávaní (napríklad pre zdravotné, sociálne, jazykové alebo iné špecifické potreby). 
    
                        
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
        
        
            0/500
        
    
    Tu môžete uviesť doplňujúce informácie ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné informácie rozhodujúce pre vzdelávanie.


        

        
            
            

    
    
    
        
            
                
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

        szsZS: {
            label: &quot;Záujem o&quot;,
            showRequiredOrOptional: true,
            required: false,
            options: [
                {
                    label: &quot;Prípravný ročník&quot;,
                    hint: &quot;Dieťa so zdravotným znevýhodnením okrem detí s narušenou komunikačnou schopnosťou ľahkého stupňa alebo vývinovou poruchou ľahkého stupňa.&quot;,
                    value: &quot;PRIPRAVNY_ROCNIK&quot;,
                },
                {
                    label: &quot;Úvodný ročník&quot;,
                    hint: &quot;Pre deti s narušenou komunikačnou schopnosťou alebo vývinovou poruchou ľahkého stupňa.&quot;,
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



    
        Vybrať školy a odbory
        
            
                Kliknutím a potiahnutím zoraďte školy podľa poradia v akom sú uvedené na prihláške.
            
        
    

    

        Polia označené hviezdičkou sú povinné

        
            
                Pridané školy
            
            
                
                
            
        

        Polia označené hviezdičkou sú povinné
            Moja škola
            
                
                    Gymnázium Metodova
                
                
                    EDUID 
                    910020859
                
                
                    Metodova 2, 82108, Bratislava
                
            

            
            
            
                
                    gymnázium
                    Netalentový odbor
                
                
                    Kód odboru: 7902J00
                     • 
                    Vyučovací jazyk: slovenský
                
            

            
                
                    

    
    
    
        
            
                
                    Záujem o prípravu v systéme duálneho vzdelávania
                    (nepovinné)
                
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                
                
                    

    
    
    
        
            
                
                    Záujem o školský internát
                    (nepovinné)
                
                
            
            
        
    

                
                
                    

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                

                Odstrániť z prihlášky

            

        
        

        

        
            
                add
                Pridať odbor mojej školy
            
            
                add
                
                Pridať odbor
            
        
    

    
        infoDôležité upozorneniePoradie uvedených škôl v prihláške ovplyvní prijímací proces.

        
            Poradie škôl
        

        
            
                
                    1
                    
                        Presunúť vyššie
                        arrow_upward
                    
                    
                        Presunúť nižšie
                        arrow_downward
                    
                
                Moja škola
                
                    
                        Gymnázium Metodova
                    
                    
                        
                            gymnázium
                             • 
                            1. termín (1.kolo)
                        
                        Netalentový odbor
                    
                    
                        Metodova 2, 82108, Bratislava
                    
                
            
            
                drag_indicator
            
        
    

    

    

    

    

    

    

    

    


    

    

    

    


    

    

    


                    
                    
                        

    Zákonný zástupca dieťaťa
    
        Skontrolujte, či sa vyplnené údaje zhodujú s údajmi v papierovej prihláške. V prípade potreby ich doplňte alebo upravte.
    

    
        
         

        Polia označené hviezdičkou sú povinné

         

        
            

    
    
        Prihlášku podáva:
        (nepovinné)
    
    
    
    
                        
                        Zákonný zástupca 
                         
                        
                        Zástupca zariadenia 
                         
    

        
        
        
            
                Osobné údaje zákonného zástupcu č. 1

                
                    

    
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
            
        
    

                
                
                    

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                

    
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



                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Korešpondenčná adresa

            
                

    
    
        
        
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt rodiča z RFO  
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

            
            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca1InaAdresaKrajina: {
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
                
                    

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                    
                        

    
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
            
        
    

                    
                    
                        

    
        Rodné číslo
        (nepovinné)
    
    Zadajte vo formáte XXXXXX/XXXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    

    
    
    
        
            
                
                    Osoba nemá pridelené rodné číslo
                    (nepovinné)
                
                
            
            
        
    

                    

    
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



                    
                        

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    
                        

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                    Korešpondenčná adresa
                    
                        

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Nitra (Nitra) 1435, Nitra (Nitra), Slovenská republika 
                         Trvalý pobyt dieťaťa z RFO.  
                        
                        Iná korešpondenčná adresa 
                         
    

                    
                    
                        
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;zastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2AdresaKrajina: {
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


                    
                
                        
                        Druhý zákonný zástupca nie je známy 
                         Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.  
    

                
                

            

            
                info
                
                    Podľa RFO je rodič evidovaný ako zosnulý:
                
            

            
                

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                

    
    
    
        
            
                
                    Zákonní zástupcovia žiadajú, a dokladujú to vyhlásením oboch zákonných zástupcov, aby sa vo veciach súvisiacich s prijímacím konaním dieťaťa komunikovalo výhradne so zákonným zástupcom č. 1.
                    (nepovinné)
                
                
            
            
        
    

            
                        
                        nie 
                         
    

            
            
            
                
                    

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                
            
        

        
            
                Údaje o zariadení zastupujúcom dieťa

                
                    

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        E-mail
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            

            Adresa zariadenia

            
                
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        
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


    



    window[&quot; , &quot;'&quot; , &quot;adresaZariadeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        adresaZariadeniaKrajina: {
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


                        
        
    

                    
                    
                        

    const controlSettingsNajstSkolu = {
        loading: &quot; , &quot;'&quot; , &quot;Načítava sa&quot; , &quot;'&quot; , &quot;
    };

    const controlSettingsInfoOZS = {
        prichodZiakaRadio : {
            label: &quot;Príchod žiaka:&quot;,
            options: [
                {
                    label: &quot;Zo ZŠ na Slovensku&quot;,
                    value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Zo školy v zahraničí&quot;,
                    value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Z praxe&quot;,
                    value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;,
                    id: &quot; , &quot;'&quot; , &quot;ddd&quot; , &quot;'&quot; , &quot;,
                    class: &quot; , &quot;'&quot; , &quot;aaa&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Iné&quot;,
                    value: &quot; , &quot;'&quot; , &quot;4&quot; , &quot;'&quot; , &quot;
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
        rocnikSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        eduidSkolyInput: {
            label: &quot;EDUID základnej školy&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikSKSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte ročník, ktorý žiak študuje.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        triedaInput: {
            label: &quot;Trieda&quot;,
            hint: &quot;Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9Á-Žá-žA-Za-z\,\.\&quot; , &quot;'&quot; , &quot;\-\s]{1,10}$/,
                    message: &quot;Môžete zadať maximálne 10 znakov.&quot;,
                    regexError: &quot;Môžete zadať maximálne 10 znakov.&quot;
                }
            ],
            required: true
        },
        rokSkolskejDochadzkySKSelect: {
            label: &quot;Rok školskej dochádzky&quot;,
            hint: &quot;Uveďte koľko rokov plní žiak školskú dochádzku.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        vyucovaciJazykVZakladnejSkoleSKAutocomplete: {
            label: &quot;Vyučovací jazyk v základnej škole&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikZPraxeSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        rocnikIneSelect: {
            label: &quot;Ročník&quot;,
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            selectError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte dodatočné informácie o základnej škole.
        
    

    
        
            
                Informácie o základnej škole
            
            Upraviť
        
        
            
                Príchod žiaka
                
            
            
                EDUID základnej školy
                
            
            
                Názov základnej školy
                
            
            
                Ročník
                
            
            
                Trieda
                
            
            
                Rok školskej dochádzky
                
            
            
                Vyučovací jazyk v základnej škole
                
            
        
    

    
        
            

    
    
        Príchod žiaka:
        (nepovinné)
    
    
    
    
                        
                        Zo ZŠ na Slovensku 
                         

            
                Názov základnej školy
                Uveďte školu, ktorú žiak navštevuje.
                
                    
                    warning
                
                Toto pole je povinné.
                
            

            
                

    
        EDUID základnej školy
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Trieda
        (nepovinné)
    
    Uveďte triedu, ktorú žiak navštevuje v tvare napríklad “6.A”.
    
        
        warning
        warning
        
            calendar_month
        
        
            visibility
        
    
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
                        
                        Zo školy v zahraničí 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte ročník, ktorý žiak študuje.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Rok školskej dochádzky
        (nepovinné)
    
    Uveďte koľko rokov plní žiak školskú dochádzku.
    
        12345678910
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

            
                

    
        Vyučovací jazyk v základnej škole
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Z praxe 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
                        
                        Iné 
                         

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
        4.5.6.7.8.9.10.
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

            

        
    

        

        

        

        

        
    

    
        
            
                
                    
                        Upraviť
                        Informácie o základnej škole
                    
                    
                        
                            close
                        
                    
                
                
                    Polia označené hviezdičkou sú povinné
                    
                
                
                    Zrušiť
                    Uložiť
                
            
        
        
    

    

                    
                    
                        

    const pageTextsVysledkyVzdelavania = {
        polrok: &quot;polrok&quot;,
        rocnik: &quot;ročník&quot;,
        vyberteHodnotenie: &quot;Vyberte hodnotenie&quot;,
        zmeniliSteUdajePanelTitle: &quot;Pre overenie správnosti údajov bude požadované:&quot;,
        zmeniliSteUdajePanelText: &quot;Zmenili ste údaje na jednom predmete / viacerých predmetoch, preto bude nutné priložiť k prihláške:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;&quot;,
        neuviedliSteUdajePanelText: &quot;&quot;,
        slovneHodnoteniePanelDescription: &quot;V prípade použitia možnosti slovné hodnotenie sa k prihláške doloží kópia vysvedčenia za daný ročník.&quot;,
        modalDescription: &quot;Vyberte predmet a hodnotenie pre %rocnik%. ročník, podľa vysvedčenia.&quot;,
        overenuKopiu: &quot;overenú kópiu vysvedčenia pre &quot;
    };

    const controlSettingsVysledkyVzdelavania = {
        hodnotenieTable: {
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        inaZnamkovaSchema: {
            label: &quot;&quot;,
            hint: &quot;&quot;,
            required: false
        },
        modalPredmet: {
            label: &quot;Predmet&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalJazyk: {
            label: &quot;Jazyk&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalHodnotenie: {
            label: &quot;Hodnotenie&quot;,
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



    Výsledky vzdelávania na základnej škole

    
        Doplňte výsledky vzdelávania z posledných 4 ročníkov základnej školy podľa vysvedčenia. V prípade, že žiak opakoval ročník, uveďte známky z posledného ročníka.
    

    
        
        
    

    
        info
        
            Dôležité
            Ak boli k prihláške priložené vysvedčenia s inou známkovacou škálou, než aká sa používa na slovenských školách, pred ich zápisom ich preveďte na štandardné známky podľa tohto dokumentu.
        
        Stiahnuť dokument
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
            
                
                    
                    
                    
                
                
                    
                        Predmet
                        polrok
                        Akcia
                    
                
                
                    
                        
                            
                                
                                    add
                                
                                
                                    Pridať predmet
                                
                            
                        
                    
                
            
        

        
    
    

    
        
            
                
                    Pridať predmet
                
                
                    
                        close
                    
                
                
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    

                    
                    
                        

    const controlSettingsSutaze = {
         modalNazovSutazeText: {
            label: &quot;Názov súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            regexError: &quot;Zadajte názov súťaže.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9%\s\S]{1,255}$/,
                    message: &quot;Zadajte názov súťaže.&quot;,
                    regexError: &quot;Zadajte názov súťaže.&quot;
                }
            ],
            required: true

        },
        modalDruhSutazeSelect: {
            label: &quot;Druh súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalDruhSportuSelect: {
            label: &quot;Druh športu&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalUrovenSutazeSelect: {
            label: &quot;Úroveň súťaže&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
        },
        modalTypUmiestneniaRadio : {
            label: &quot;Typ umiestnenia&quot;,
            options: [
                // {
                //     label: &quot;1. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;2. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;3. miesto&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;3&quot; , &quot;'&quot; , &quot;
                // },
                // {
                //     label: &quot;Bez umiestnenia&quot;,
                //     value: &quot; , &quot;'&quot; , &quot;99&quot; , &quot;'&quot; , &quot;
                // },
                {
                    label: &quot;Iné&quot;,
                    value: &quot; , &quot;'&quot; , &quot;5&quot; , &quot;'&quot; , &quot;
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

        modalTypUmiestneniaIneText: {
            label: &quot;Zadajte umiestnenie&quot;,
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
        modalSkolskyRokSelect: {
            label: &quot;Školský rok&quot;,
            hint: &quot;V ktorom sa žiak zúčastnil súťaže.&quot;,
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

    const pageTextsSutaze = {
        vzdelavanieVJazykuLabelMS: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
        skolskyRok: &quot; , &quot;'&quot; , &quot;Školský rok&quot; , &quot;'&quot; , &quot;,
        uroven: &quot; , &quot;'&quot; , &quot;úroveň&quot; , &quot;'&quot; , &quot;,
    };



    Súťaže

    
        Uveďte súťaže, v ktorých žiak dosiahol výborné výsledky alebo umiestnenia.
    

    
        
            
                
                    Súťaž
                    Umiestnenie
                    Akcia
                
            
            
                
                    
                        
                            
                                add
                            
                            
                                Pridať súťaž
                            
                        
                    
                
            
        
    

    

    
        
            
                
                    
                        Pridať súťaž
                    
                    
                        Pokiaľ pridávate súťaž, bude povinné priložiť diplom o úspešnom absolvovaní.
                    
                
                
                    
                        close
                    
                
            
            
                Polia označené hviezdičkou sú povinné
                
                    

    
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
                
                    

    
    
        
        
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
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                
            
            
                Zrušiť
                
                    Pridať
                
            
        
        
    


                    
                    
                        

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
                    functionOrRegex: /^$|^.{1,500}$/,
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
        skolyHeaderSS: &quot;Stredná škola&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaInfoHeaderSS: &quot;Stredná škola č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;,
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
        zariadenieHeader: &quot;Údaje o zariadení zastupujúcom dieťa&quot;,
        
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
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
            
            
                Adresa miesta, kde sa dieťa obvykle zdržiava, ak sa nezdržiava na adrese trvalého pobytu
                -
            
            
                Povinné predprimárne vzdelávanie aktuálne v
                -
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        
                    
                    
                        Požadovaná výchova
                        
                    
                    
                        Záujem o stravovanie v jedálni
                        
                    
                    
                        Záujem o Školský klub detí
                        
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        
                    
                    
                        Popis znevýhodnenia / nadania
                        
                    
                    
                        Poznámka
                        
                    
                
            
            
                
                    
                        Zmenená pracovná schopnosť
                        -
                    
                    
                        Upravené podmienky prijímacieho konania (Špeciálne výchovno-vzdelávacie potreby)
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
                    
                    
                        Číslo elektronickej schránky
                        -
                    
                    
                        Adresa zariadenia
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
      <webElementGuid>00a1c919-fde9-4f6c-8131-94122141d270</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
