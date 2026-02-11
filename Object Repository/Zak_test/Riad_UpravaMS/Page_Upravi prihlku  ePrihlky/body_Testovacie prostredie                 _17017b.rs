<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Testovacie prostredie                 _17017b</name>
   <tag></tag>
   <elementGuidId>7b458b6d-ea6c-477e-b559-74e60e6d09de</elementGuidId>
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
      <webElementGuid>3e0b0273-acfb-4824-845a-091dbc9b9418</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-frontend-supported</value>
      <webElementGuid>2fc33796-a383-4ce4-8621-469e00c83a89</webElementGuid>
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
                                    
                                
                    
                
            
        
    

    

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        zastupca1AdresaRadio: {
            options: [],
            direction: 'column',
            required: true,
        },
        zastupca1Meno: {
            label: 'Meno',
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            element: 'zastupca1Meno',
        },
        zastupca1Priezvisko: {
            label: 'Priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            element: 'zastupca1Priezvisko',
        },
        zastupca1DatumNarodenia: {
            label: 'Dátum narodenia',
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
            element: 'zastupca1DatumNarodenia',
        },
        zastupca1CisloSchranky: {
            label: 'Číslo elektronickej schránky',
            regexError: 'Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.',
        },
        zastupca1Email: {
            label: 'Email',
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca1Telefon: {
            label: 'Telefónne číslo',
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        nazovZariadenia: {
            label: 'Názov zariadenia',
            regexError: &quot;Zadajte názov zariadenia.&quot;,
            required: false,
            icoChybaMessage: 'Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia'
        },
        icoZariadenia: {
            label: 'IČO zariadenia',
            regexError: &quot;Zadajte IČO zariadenia ako 8 alebo 12-miestne číslo.&quot;,
            required: false,
            nazovZariadeniaChybaMessage: 'Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia'

        },
        zastupca2Radio: {
            label: 'Vyberte jednu z možností',
            options: [
                {
                    label: &quot;Druhý zákonný zástupca je známy&quot;,
                    value: &quot;ZNAMY&quot;,
                    hint: 'Vyplňte údaje druhého zákonného zástupcu.'
                },
                {
                    label: &quot;Druhý zákonný zástupca nie je známy&quot;,
                    value: &quot;NEZNAMY&quot;,
                    hint: 'Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.'
                }
            ],
            direction: 'column',
        },
        zastupca2Meno: {
            label: 'Meno',
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        zastupca2Priezvisko: {
            label: 'Priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        zastupca2DatumNarodenia: {
            label: 'Dátum narodenia',
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        zastupca2CisloSchranky: {
            label: 'Číslo elektronickej schránky',
            regexError: 'Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.',
        },
        zastupca2Email: {
            label: 'Email',
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca2Telefon: {
            label: 'Telefónne číslo',
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        zastupca2AdresaRadio: {
            label: '',
            options: [],
            direction: 'column',
        },
        agreementRadio: {
            label: 'Súhlas druhého zákonného zástupcu',
            options: [
                {
                    label: &quot;áno&quot;,
                    value: &quot;ANO&quot;
                },
                {
                    label: &quot;nie&quot;,
                    value: &quot;NIE&quot;
                }
            ],
            direction: 'column',
        },
        noAgreementReason: {
            label: 'Dôvod nezabezpečenia súhlasu',
            required: true
        }
    };



    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90345.01
    

    
        Zástupca dieťaťa
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

            
            
                

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Lučenec (Lučenec)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Gemerská cesta
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
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                
                    

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Poprad (Poprad)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Námestie Svätého Egídia
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
    

            
            
        

        
            

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                        
                        nie 
                         
    

        
        
            
                

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        Zdravotný - nevie sa podpísať
        warning
        
            keyboard_arrow_down
        
    


            
        
    
    
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
      <webElementGuid>75f7b2b2-d196-4ce5-9b88-a8eede0a2150</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;govuk-frontend-supported&quot;]</value>
      <webElementGuid>5c7b2889-e476-4734-a1d1-15f1663482b7</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>cd5efe95-e50a-414c-a9f6-9463a64d21a3</webElementGuid>
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
                                    
                                
                    
                
            
        
    

    

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        zastupca1AdresaRadio: {
            options: [],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            required: true,
        },
        zastupca1Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1Meno&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1Priezvisko&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1DatumNarodenia&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1CisloSchranky: {
            label: &quot; , &quot;'&quot; , &quot;Číslo elektronickej schránky&quot; , &quot;'&quot; , &quot;,
            regexError: &quot; , &quot;'&quot; , &quot;Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1Email: {
            label: &quot; , &quot;'&quot; , &quot;Email&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca1Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        nazovZariadenia: {
            label: &quot; , &quot;'&quot; , &quot;Názov zariadenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte názov zariadenia.&quot;,
            required: false,
            icoChybaMessage: &quot; , &quot;'&quot; , &quot;Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia&quot; , &quot;'&quot; , &quot;
        },
        icoZariadenia: {
            label: &quot; , &quot;'&quot; , &quot;IČO zariadenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte IČO zariadenia ako 8 alebo 12-miestne číslo.&quot;,
            required: false,
            nazovZariadeniaChybaMessage: &quot; , &quot;'&quot; , &quot;Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia&quot; , &quot;'&quot; , &quot;

        },
        zastupca2Radio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte jednu z možností&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;Druhý zákonný zástupca je známy&quot;,
                    value: &quot;ZNAMY&quot;,
                    hint: &quot; , &quot;'&quot; , &quot;Vyplňte údaje druhého zákonného zástupcu.&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Druhý zákonný zástupca nie je známy&quot;,
                    value: &quot;NEZNAMY&quot;,
                    hint: &quot; , &quot;'&quot; , &quot;Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        zastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        zastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        zastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        zastupca2CisloSchranky: {
            label: &quot; , &quot;'&quot; , &quot;Číslo elektronickej schránky&quot; , &quot;'&quot; , &quot;,
            regexError: &quot; , &quot;'&quot; , &quot;Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.&quot; , &quot;'&quot; , &quot;,
        },
        zastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;Email&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        zastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            options: [],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        agreementRadio: {
            label: &quot; , &quot;'&quot; , &quot;Súhlas druhého zákonného zástupcu&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;áno&quot;,
                    value: &quot;ANO&quot;
                },
                {
                    label: &quot;nie&quot;,
                    value: &quot;NIE&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        noAgreementReason: {
            label: &quot; , &quot;'&quot; , &quot;Dôvod nezabezpečenia súhlasu&quot; , &quot;'&quot; , &quot;,
            required: true
        }
    };



    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90345.01
    

    
        Zástupca dieťaťa
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

            
            
                

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Lučenec (Lučenec)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Gemerská cesta
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
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                
                    

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Poprad (Poprad)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Námestie Svätého Egídia
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
    

            
            
        

        
            

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                        
                        nie 
                         
    

        
        
            
                

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        Zdravotný - nevie sa podpísať
        warning
        
            keyboard_arrow_down
        
    


            
        
    
    
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
                                    
                                
                    
                
            
        
    

    

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        zastupca1AdresaRadio: {
            options: [],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            required: true,
        },
        zastupca1Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1Meno&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1Priezvisko&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
            element: &quot; , &quot;'&quot; , &quot;zastupca1DatumNarodenia&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1CisloSchranky: {
            label: &quot; , &quot;'&quot; , &quot;Číslo elektronickej schránky&quot; , &quot;'&quot; , &quot;,
            regexError: &quot; , &quot;'&quot; , &quot;Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.&quot; , &quot;'&quot; , &quot;,
        },
        zastupca1Email: {
            label: &quot; , &quot;'&quot; , &quot;Email&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca1Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        nazovZariadenia: {
            label: &quot; , &quot;'&quot; , &quot;Názov zariadenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte názov zariadenia.&quot;,
            required: false,
            icoChybaMessage: &quot; , &quot;'&quot; , &quot;Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia&quot; , &quot;'&quot; , &quot;
        },
        icoZariadenia: {
            label: &quot; , &quot;'&quot; , &quot;IČO zariadenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte IČO zariadenia ako 8 alebo 12-miestne číslo.&quot;,
            required: false,
            nazovZariadeniaChybaMessage: &quot; , &quot;'&quot; , &quot;Ak je zákonný zástupca právnickou osobou, musí mať uvedený Názov zariadenia aj IČO zariadenia&quot; , &quot;'&quot; , &quot;

        },
        zastupca2Radio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte jednu z možností&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;Druhý zákonný zástupca je známy&quot;,
                    value: &quot;ZNAMY&quot;,
                    hint: &quot; , &quot;'&quot; , &quot;Vyplňte údaje druhého zákonného zástupcu.&quot; , &quot;'&quot; , &quot;
                },
                {
                    label: &quot;Druhý zákonný zástupca nie je známy&quot;,
                    value: &quot;NEZNAMY&quot;,
                    hint: &quot; , &quot;'&quot; , &quot;Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        zastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;
        },
        zastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;
        },
        zastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.8.2000). &quot;,
        },
        zastupca2CisloSchranky: {
            label: &quot; , &quot;'&quot; , &quot;Číslo elektronickej schránky&quot; , &quot;'&quot; , &quot;,
            regexError: &quot; , &quot;'&quot; , &quot;Zadajte číslo elektronickej schránky v tvare &quot;E&quot; + 10 numerických znakov.&quot; , &quot;'&quot; , &quot;,
        },
        zastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;Email&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: false
        },
        zastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            required: false
        },
        zastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
            options: [],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        agreementRadio: {
            label: &quot; , &quot;'&quot; , &quot;Súhlas druhého zákonného zástupcu&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;áno&quot;,
                    value: &quot;ANO&quot;
                },
                {
                    label: &quot;nie&quot;,
                    value: &quot;NIE&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },
        noAgreementReason: {
            label: &quot; , &quot;'&quot; , &quot;Dôvod nezabezpečenia súhlasu&quot; , &quot;'&quot; , &quot;,
            required: true
        }
    };



    

    
                        
                            Správa prihlášok
                        
        Upraviť prihlášku
    




    
                
                    ...
                    
                                    
                                        Správa prihlášok
                                    
                    
                
            Upraviť prihlášku
    

    
    
        Upraviť prihlášku
        P-2025-90345.01
    

    
        Zástupca dieťaťa
        Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.
    

    
        Informácie o zákonnom zástupcovi č. 1

        
            
                

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

            
            
                

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        Názov zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
            
                

    
        IČO zariadenia
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

            
        

        Adresa bydliska

        
            
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Lučenec (Lučenec)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Gemerská cesta
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
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známy 
                         Vyplňte údaje druhého zákonného zástupcu.  
                
                    

    
        Meno
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Priezvisko
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
        
    
    «F Y»PoUtStŠtPiSoNe                                          Clear date    

                
                
                    

    
        Číslo elektronickej schránky
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Email
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                
                    

    
        Telefónne číslo
        (nepovinné)
    
    
    
        
        warning
        warning
        
            calendar_month
        
    
    

                
                Adresa bydliska
                
                    
    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        Slovenská republika
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Obec
        (nepovinné)
    
    
    
        
        Poprad (Poprad)
        warning
        
            keyboard_arrow_down
        
    


    
    
        

    
        Ulica
        (nepovinné)
    
    
    
        
        Námestie Svätého Egídia
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
    

            
            
        

        
            

    
    
        Súhlas druhého zákonného zástupcu
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
                        
                        nie 
                         
    

        
        
            
                

    
        Dôvod nezabezpečenia súhlasu
        (nepovinné)
    
    
    
        
        Zdravotný - nevie sa podpísať
        warning
        
            keyboard_arrow_down
        
    


            
        
    
    
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
      <webElementGuid>957f0ff6-16b3-41a6-8298-9130d4db36f0</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
