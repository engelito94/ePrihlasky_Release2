<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Veobecn informcie                      _1aa588</name>
   <tag></tag>
   <elementGuidId>2e5b065a-c6ae-424c-8bc4-858181c893f8</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#podrobnosti-o-dietati</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@class = 'autocomplete-dropdown']</value>
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
      <webElementGuid>19cd95fe-50a4-4d5c-a723-18e156fa7ccc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>podrobnosti-o-dietati</value>
      <webElementGuid>1b47814b-1880-4032-9f28-edee4347fe26</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>form-section-container</value>
      <webElementGuid>0140653c-8113-4d67-964a-3021fc451868</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        kórejský
        warning
        
            keyboard_arrow_down
        
    
kórejský

                

            

            
                Trvalý pobyt
                
    
        

    
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
        
    
    

                
            

        </value>
      <webElementGuid>4d049216-cc38-4d43-87e0-773256cdd155</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;podrobnosti-o-dietati&quot;)</value>
      <webElementGuid>2a787bc9-3645-4fbe-b9d1-25564e89e955</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='podrobnosti-o-dietati']</value>
      <webElementGuid>2a0a03c1-15ae-421d-908e-e552b065f7ea</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='pridanie-dietata-upravit']/div[6]</value>
      <webElementGuid>d1437139-0a19-4fdb-ba91-c783b299df19</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Skontrolujte, či sa údaje uvedené v RFO zhodujú s údajmi v papierovej prihláške.'])[1]/following::div[1]</value>
      <webElementGuid>09000014-683d-4b2b-b9a4-a3afe2cb2a60</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Podrobnosti o dieťati'])[1]/following::div[2]</value>
      <webElementGuid>0c04f480-afe8-4bd6-b8ee-7cc0033ab246</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[6]</value>
      <webElementGuid>b45cae46-84a4-4d40-9352-75dbc6ceb8d4</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'podrobnosti-o-dietati' and (text() = concat(&quot;
            
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        kórejský
        warning
        
            keyboard_arrow_down
        
    
kórejský

                

            

            
                Trvalý pobyt
                
    
        

    
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
        
    
    

                
            

        &quot;) or . = concat(&quot;
            
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
    
    
    
        
        slovenský
        warning
        
            keyboard_arrow_down
        
    


                
                
                    

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        kórejský
        warning
        
            keyboard_arrow_down
        
    
kórejský

                

            

            
                Trvalý pobyt
                
    
        

    
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
        
    
    

                
            

        &quot;))]</value>
      <webElementGuid>a32415b4-c7c4-401f-b279-5577ccb1e8f3</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
