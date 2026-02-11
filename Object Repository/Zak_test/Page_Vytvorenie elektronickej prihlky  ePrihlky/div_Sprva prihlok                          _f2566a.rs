<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Sprva prihlok                          _f2566a</name>
   <tag></tag>
   <elementGuidId>e7d85b10-4bd9-43fc-9ccc-71f00c22267f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.privatna-zona-content</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='ziadost']/div/div[2]</value>
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
      <webElementGuid>ff9a3bcb-577f-4a7d-bc63-b2c3bdb3051f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>privatna-zona-content</value>
      <webElementGuid>19c06372-ffe8-4392-8410-e9da839d69ce</webElementGuid>
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
    

                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Zadajte údaje dieťaťa z papierovej prihlášky.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: 'Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne',
        prosimCakajte: 'Prosím, čakajte',
        systemPracuje: 'Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.',
        '8020': &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        '8021': &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        'dietaNemaInyMaterinsky': &quot;Dieťa nemá iný materinský jazyk&quot;
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má dieťa rodné číslo?&quot;,
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
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
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




    Pridať dieťa

    
        
            Pridať dieťa podľa rodného čísla.
        

        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
            
                

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    
Toto pole je povinné.
            
        
    
    
        
            
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
    
        

    
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
        
    
    

                    
                

            
        
    


                    
                    
                        

    Zástupca dieťaťa

    
        
            Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
        
         
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


        

        

        
            Informácie o zákonnom zástupcovi č. 2
            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


                
            

        

        
            info
            
                Podľa RFO je rodič evidovaný ako zosnulý:
            
        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
            
                

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
        
    

                    
                    
                        

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
            label: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním&quot;,
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
            direction: 'row',
            povinneError: &quot;Toto pole je povinné.&quot;,
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
            direction: 'row',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
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
            direction: 'row',
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
            direction: 'row',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia / nadania&quot;,
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
            hint: &quot;Tu môžete uviesť doplňujúce informácie (ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo ak žiadate o individuálne vzdelávanie).&quot;,
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
        }
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách dieťaťa.
    

    
        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            
            
                

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


            
        

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const pageTextsPrilohy = {
        '8106': 'Nahrávanie sa nepodarilo. Skúste to ešte raz.',
        '8105': 'Dokument bol úspešne nahratý.',
        '8104': 'Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.',
        '8113': 'Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.',
        '8108': 'Dokument bol úspešne vymazaný.',
        '8109': 'Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.',
        '8110': 'Všetky potrebné prílohy boli pridané.',
        'ziadnePovinnePrilohy': 'Nie sú potrebné žiadne povinné prílohy.'
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect : {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
        
            



        
    
        
            Iné povinné prílohy pre špecifické prípady:
            (Rozbaľte sekcie, ktoré sa vás týkajú.)
        
        
            
                



            
        

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning        
    

        
        
        
        

        
            
                
                
                
                
            
            
        

        
            
            

            
                
                    cloud_upload
                    
                        Nahrajte súbor
                    
                    alebo ho sem potiahnite (max. 10 MB)
                    
                        add
                        Vybrať súbor
                    
                
            
        

        
            
                article_rounded
                error
                
                    
                    
                
                
                
                clear_rounded
            
        

        
            Nahrané súbory
            
        

        
            warning_rounded
            
                Zatiaľ neboli nahrané žiadne povinné prílohy. Prosím, nahrajte nasledujúce prílohy:
                
                
            
        

        
            info
            
                Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.
            
        

        
            check_circle
            
                Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.
            
        

        
            check_circle
            
                Prosím, nahrajte všetky povinné prílohy.
                
                    
                        Nahrané prílohy:
                        
                    
                    
                        Chýbajúce prílohy:
                        
                    
                
            
        
    

                    
                    
                        

    const controlSettingsOU = {
        OUDatumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
            placeholder: 'DD.MM.YYYY',
            datepicker: true,
            attributes: {
                maxLength: 10
            },
            validators: [
                {
                    type: 'required',
                    message: &quot;Toto pole je povinné.&quot;,
                },
                {
                    type: 'regex',
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])[.\/-](0?[1-9]|1[0-2])[.\/-]([0-9]{4})$/,
                    message: &quot;Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.8.2025).&quot;,
                }
            ],
            dpAktualnyRokMessage: &quot;Zadajte dátum z aktuálneho roka.&quot;,
            dpStarsiDatumMessage: &quot;Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.&quot;
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
        }
    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    
        Dátum prihlášky

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        

        Poznámka k prihláške

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;
    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        Podrobnosti o prihláške
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Poznámka školy
                -
            
        
    

    
        Podrobnosti o dieťati
        
            
                Meno a priezvisko dieťaťa
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
            
        
    

    
        Informácie o škole
        
        
    

    
        Podrobnosti o zákonných zástupcoch dieťaťa
        
            Informácie o zákonnom zástupcovi č.1:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Názov zariadenia
                    -
                
                
                    IČO zariadenia
                    -
                
            
            
            Informácie o zákonnom zástupcovi č.2:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Súhlas druhého zákonného zástupcu
                    -
                
                
                    Dôvod nezabezpečenia súhlasu
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        Prílohy a doplňujúce informácie o dieťati
        
            Prílohy

            

            

            

            Doplňujúce informácie o dieťati
            
                
                    Žiadam o prijatie dieťaťa na
                    -
                
                
                    Požadovaná výchova
                    -
                
                
                    Záujem o stravovanie v jedálni
                    -
                
                
                    Záujem o Školský klub detí
                    -
                
                
                    Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                    -
                
                
                    Popis znevýhodnenia / nadania
                    -
                
                
                    Poznámka
                    -
                
            
        
    





                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                
            
        </value>
      <webElementGuid>0e5a835d-55f7-49e1-9348-33557b75df8a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;ziadost&quot;)/div[@class=&quot;menu-layout&quot;]/div[@class=&quot;privatna-zona-content&quot;]</value>
      <webElementGuid>6388bc1d-c6e4-441f-99d8-e47972cddeeb</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='ziadost']/div/div[2]</value>
      <webElementGuid>58538b07-a9bf-485b-8b53-bf2fbbad6220</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Súhrnný prehľad'])[1]/following::div[2]</value>
      <webElementGuid>4ca4b8c3-5a11-4497-a76b-14c3ebb83f41</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Ostatné údaje'])[1]/following::div[2]</value>
      <webElementGuid>94355cce-c9bf-4640-9fb7-3e42060dbf01</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div[2]</value>
      <webElementGuid>fc333065-b662-4952-8218-a57699dc92e8</webElementGuid>
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
    

                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Zadajte údaje dieťaťa z papierovej prihlášky.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: &quot; , &quot;'&quot; , &quot;Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne&quot; , &quot;'&quot; , &quot;,
        prosimCakajte: &quot; , &quot;'&quot; , &quot;Prosím, čakajte&quot; , &quot;'&quot; , &quot;,
        systemPracuje: &quot; , &quot;'&quot; , &quot;Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;dietaNemaInyMaterinsky&quot; , &quot;'&quot; , &quot;: &quot;Dieťa nemá iný materinský jazyk&quot;
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má dieťa rodné číslo?&quot;,
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
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
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




    Pridať dieťa

    
        
            Pridať dieťa podľa rodného čísla.
        

        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
            
                

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    
Toto pole je povinné.
            
        
    
    
        
            
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
    
        

    
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
        
    
    

                    
                

            
        
    


                    
                    
                        

    Zástupca dieťaťa

    
        
            Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
        
         
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


        

        

        
            Informácie o zákonnom zástupcovi č. 2
            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


                
            

        

        
            info
            
                Podľa RFO je rodič evidovaný ako zosnulý:
            
        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
            
                

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
        
    

                    
                    
                        

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
            label: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním&quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia / nadania&quot;,
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
            hint: &quot;Tu môžete uviesť doplňujúce informácie (ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo ak žiadate o individuálne vzdelávanie).&quot;,
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
        }
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách dieťaťa.
    

    
        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            
            
                

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


            
        

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const pageTextsPrilohy = {
        &quot; , &quot;'&quot; , &quot;8106&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8105&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne nahratý.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8104&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8113&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8108&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne vymazaný.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8109&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8110&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli pridané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ziadnePovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nie sú potrebné žiadne povinné prílohy.&quot; , &quot;'&quot; , &quot;
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect : {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
        
            



        
    
        
            Iné povinné prílohy pre špecifické prípady:
            (Rozbaľte sekcie, ktoré sa vás týkajú.)
        
        
            
                



            
        

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning        
    

        
        
        
        

        
            
                
                
                
                
            
            
        

        
            
            

            
                
                    cloud_upload
                    
                        Nahrajte súbor
                    
                    alebo ho sem potiahnite (max. 10 MB)
                    
                        add
                        Vybrať súbor
                    
                
            
        

        
            
                article_rounded
                error
                
                    
                    
                
                
                
                clear_rounded
            
        

        
            Nahrané súbory
            
        

        
            warning_rounded
            
                Zatiaľ neboli nahrané žiadne povinné prílohy. Prosím, nahrajte nasledujúce prílohy:
                
                
            
        

        
            info
            
                Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.
            
        

        
            check_circle
            
                Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.
            
        

        
            check_circle
            
                Prosím, nahrajte všetky povinné prílohy.
                
                    
                        Nahrané prílohy:
                        
                    
                    
                        Chýbajúce prílohy:
                        
                    
                
            
        
    

                    
                    
                        

    const controlSettingsOU = {
        OUDatumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
            placeholder: &quot; , &quot;'&quot; , &quot;DD.MM.YYYY&quot; , &quot;'&quot; , &quot;,
            datepicker: true,
            attributes: {
                maxLength: 10
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;,
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])[.\/-](0?[1-9]|1[0-2])[.\/-]([0-9]{4})$/,
                    message: &quot;Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.8.2025).&quot;,
                }
            ],
            dpAktualnyRokMessage: &quot;Zadajte dátum z aktuálneho roka.&quot;,
            dpStarsiDatumMessage: &quot;Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.&quot;
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
        }
    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    
        Dátum prihlášky

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        

        Poznámka k prihláške

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;
    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        Podrobnosti o prihláške
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Poznámka školy
                -
            
        
    

    
        Podrobnosti o dieťati
        
            
                Meno a priezvisko dieťaťa
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
            
        
    

    
        Informácie o škole
        
        
    

    
        Podrobnosti o zákonných zástupcoch dieťaťa
        
            Informácie o zákonnom zástupcovi č.1:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Názov zariadenia
                    -
                
                
                    IČO zariadenia
                    -
                
            
            
            Informácie o zákonnom zástupcovi č.2:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Súhlas druhého zákonného zástupcu
                    -
                
                
                    Dôvod nezabezpečenia súhlasu
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        Prílohy a doplňujúce informácie o dieťati
        
            Prílohy

            

            

            

            Doplňujúce informácie o dieťati
            
                
                    Žiadam o prijatie dieťaťa na
                    -
                
                
                    Požadovaná výchova
                    -
                
                
                    Záujem o stravovanie v jedálni
                    -
                
                
                    Záujem o Školský klub detí
                    -
                
                
                    Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                    -
                
                
                    Popis znevýhodnenia / nadania
                    -
                
                
                    Poznámka
                    -
                
            
        
    





                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                
            
        &quot;) or . = concat(&quot;
            
                
                    

    
                        
                            Správa prihlášok
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    
                    
                        

    const pageTextsDieta = {
        sectionManualneSubHeaderText: &quot;Zadajte údaje dieťaťa z papierovej prihlášky.&quot;,
        sectionEditSubHeaderText: &quot;Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.&quot;,
        rodneCisloSaNepodariloOverit: &quot; , &quot;'&quot; , &quot;Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne&quot; , &quot;'&quot; , &quot;,
        prosimCakajte: &quot; , &quot;'&quot; , &quot;Prosím, čakajte&quot; , &quot;'&quot; , &quot;,
        systemPracuje: &quot; , &quot;'&quot; , &quot;Systém pracuje na spracovaní vašej požiadavky. Tento proces môže chvíľu trvať.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8020&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte jazyk po zadaní prvých 3 znakov a vyberte jazyk zo zoznamu zobrazených jazykov!&quot;,
        &quot; , &quot;'&quot; , &quot;8021&quot; , &quot;'&quot; , &quot;: &quot;Vyhľadajte národnosť po zadaní prvých 3 znakov a vyberte národnosť zo zoznamu zobrazených národností! &quot;,
        &quot; , &quot;'&quot; , &quot;dietaNemaInyMaterinsky&quot; , &quot;'&quot; , &quot;: &quot;Dieťa nemá iný materinský jazyk&quot;
    };

    const controlSettings = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;(nepovinné)&quot;,
        maDietaRCRadio: {
            label: &quot;Má dieťa rodné číslo?&quot;,
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
            required: true,
            rcMessage: &quot;Rodné číslo nie je správne uvedené, nie je možné vyhľadať dieťa. Upravte rodné číslo na správny formát alebo ho vymažte a zadajte údaje dieťaťa manuálne.&quot;,
        },
        rodneCisloInfo: {
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




    Pridať dieťa

    
        
            Pridať dieťa podľa rodného čísla.
        

        
            
                

    
    
        Má dieťa rodné číslo?
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

            
            
                

    
        Rodné číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    
Toto pole je povinné.
            
        
    
    
        
            
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Štátna príslušnosť
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                

                
                    
                        Trvalý pobyt
                    
                    
    
        

    
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
        
    
    

                    
                

            
        
    


                    
                    
                        

    Zástupca dieťaťa

    
        
            Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
        
         
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


        

        

        
            Informácie o zákonnom zástupcovi č. 2
            
                

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

            
            
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    

    
    
        
        
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
        
    
    

        
        
             
            
                /
            
        
        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        
    

    
        

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

    

    
        

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


    



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


                
            

        

        
            info
            
                Podľa RFO je rodič evidovaný ako zosnulý:
            
        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        
        
            
                

    
        
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
        
    

                    
                    
                        

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
            label: &quot;Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním&quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
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
            direction: &quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        DPDPopisSVVText: {
            label: &quot;Popis znevýhodnenia / nadania&quot;,
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
            hint: &quot;Tu môžete uviesť doplňujúce informácie (ako napríklad stravovacie obmedzenia, intolerancie, alergie alebo ak žiadate o individuálne vzdelávanie).&quot;,
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
        }
    };



    Doplňujúce informácie o dieťati

    
        Vyplňte dodatočné informácie o potrebách dieťaťa.
    

    
        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            

    
    
        
        
        (nepovinné)
    
    
    
    
    
    

        

        
            
            
                

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


            
        

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const pageTextsPrilohy = {
        &quot; , &quot;'&quot; , &quot;8106&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8105&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne nahratý.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8104&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nahrávanie sa nepodarilo z dôvodu veľkosti dokumentu. Skúste to ešte raz.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8113&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vložili ste nepovolený formát súboru. Povolené sú %p_ZoznamFormatov%.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8108&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Dokument bol úspešne vymazaný.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8109&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky povinné prílohy boli nahradené. Môžete pokračovať ďalej.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8110&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Všetky potrebné prílohy boli pridané.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ziadnePovinnePrilohy&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nie sú potrebné žiadne povinné prílohy.&quot; , &quot;'&quot; , &quot;
    };

    const controlSettingsPrilohy = {
        TypPrilohySelect : {
            label: &quot;Vyberte typ prílohy&quot;,
            required: true,
            type: &quot;select&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
        }
    };



    Pridať prílohy

    
        
            Priložte všetky potrebné prílohy.
        
    

    
        
            



        
    
        
            Iné povinné prílohy pre špecifické prípady:
            (Rozbaľte sekcie, ktoré sa vás týkajú.)
        
        
            
                



            
        

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning        
    

        
        
        
        

        
            
                
                
                
                
            
            
        

        
            
            

            
                
                    cloud_upload
                    
                        Nahrajte súbor
                    
                    alebo ho sem potiahnite (max. 10 MB)
                    
                        add
                        Vybrať súbor
                    
                
            
        

        
            
                article_rounded
                error
                
                    
                    
                
                
                
                clear_rounded
            
        

        
            Nahrané súbory
            
        

        
            warning_rounded
            
                Zatiaľ neboli nahrané žiadne povinné prílohy. Prosím, nahrajte nasledujúce prílohy:
                
                
            
        

        
            info
            
                Zatiaľ neboli nahrané žiadne prílohy. Môžete ich pridať teraz alebo priniesť na zápis.
            
        

        
            check_circle
            
                Všetky povinné prílohy boli nahrané. Môžete pokračovať ďalej.
            
        

        
            check_circle
            
                Prosím, nahrajte všetky povinné prílohy.
                
                    
                        Nahrané prílohy:
                        
                    
                    
                        Chýbajúce prílohy:
                        
                    
                
            
        
    

                    
                    
                        

    const controlSettingsOU = {
        OUDatumPodaniaPrihlasky: {
            label: &quot;Dátum podania prihlášky&quot;,
            required: true,
            placeholder: &quot; , &quot;'&quot; , &quot;DD.MM.YYYY&quot; , &quot;'&quot; , &quot;,
            datepicker: true,
            attributes: {
                maxLength: 10
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot;Toto pole je povinné.&quot;,
                },
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^(0?[1-9]|[12][0-9]|3[01])[.\/-](0?[1-9]|1[0-2])[.\/-]([0-9]{4})$/,
                    message: &quot;Dátum prihlášky musí byť v tvare DD.MM.RRRR (napr. 10.8.2025).&quot;,
                }
            ],
            dpAktualnyRokMessage: &quot;Zadajte dátum z aktuálneho roka.&quot;,
            dpStarsiDatumMessage: &quot;Zadaný dátum musí byť aktuálny dátum alebo starší ako aktuálny dátum.&quot;
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
        }
    };



    Ostatné údaje

    
        Zadajte dátum podania a pridajte poznámku školy k prihláške.
    

    
        Dátum prihlášky

        
            

    
        
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

        

        Poznámka k prihláške

        
            

    
        
        (nepovinné)
    
    
        warning
        
        
            0/100
        
    
    


        
    

                    
                    
                        

    const controlSettingsSuhrnnyPrehlad = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        ano: &quot;Áno&quot;,
        nie: &quot;Nie&quot;,
        skolaInfoHeader: &quot;Informácie o škole č.&quot;,
        skolaNazov: &quot;Názov školy&quot;,
        skolaUlica: &quot;Adresa&quot;,
        skolaJazyk: &quot;Vo vyučovacom jazyku&quot;,
        potvrdeniaInfo: &quot;Potvrďte správnosť a pravdivosť údajov a súhlas so spracúvaním osobných údajov&quot;,
        ziadnePrilohy: &quot;Neboli nahrané žiadne prílohy.&quot;,
        poldennaVychova: &quot;poldennú výchovu a vzdelávanie&quot;,
        celodennaVychova: &quot;celodennú výchovu a vzdelávanie&quot;
    };




    Súhrnný prehľad
    
        
            Pred pridaním prihlášky skontrolujte všetky údaje vo formulári.
        
    

    
        Podrobnosti o prihláške
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Poznámka školy
                -
            
        
    

    
        Podrobnosti o dieťati
        
            
                Meno a priezvisko dieťaťa
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
            
        
    

    
        Informácie o škole
        
        
    

    
        Podrobnosti o zákonných zástupcoch dieťaťa
        
            Informácie o zákonnom zástupcovi č.1:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Názov zariadenia
                    -
                
                
                    IČO zariadenia
                    -
                
            
            
            Informácie o zákonnom zástupcovi č.2:
            
                
                    Meno a priezvisko
                    -
                
                
                    Dátum narodenia
                    -
                
                
                    Číslo elektronickej schránky
                    -
                
                
                    Adresa bydliska
                    -
                
                
                    Email
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Súhlas druhého zákonného zástupcu
                    -
                
                
                    Dôvod nezabezpečenia súhlasu
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        Prílohy a doplňujúce informácie o dieťati
        
            Prílohy

            

            

            

            Doplňujúce informácie o dieťati
            
                
                    Žiadam o prijatie dieťaťa na
                    -
                
                
                    Požadovaná výchova
                    -
                
                
                    Záujem o stravovanie v jedálni
                    -
                
                
                    Záujem o Školský klub detí
                    -
                
                
                    Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                    -
                
                
                    Popis znevýhodnenia / nadania
                    -
                
                
                    Poznámka
                    -
                
            
        
    





                    
                    
                        Späť
                         
                            Ďalej
                            Pridať prihlášku
                        
                    
                
            
        &quot;))]</value>
      <webElementGuid>ac00d090-1d9c-450d-8202-20ea556aed86</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
