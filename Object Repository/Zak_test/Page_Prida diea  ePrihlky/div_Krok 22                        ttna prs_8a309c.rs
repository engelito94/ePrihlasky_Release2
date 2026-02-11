<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Krok 22                        ttna prs_8a309c</name>
   <tag></tag>
   <elementGuidId>0b52c8d7-6092-4ca8-81ec-3b8fd36ef68f</elementGuidId>
   <selectorCollection>
      <entry>
         <key>BASIC</key>
         <value>//*[(text() = concat(&quot;
                    
                        Krok 2/2
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
        
    
Slovenská republika

                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                    Trvalý pobyt

                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        Tomášikova 5723/13, 82101, Bratislava, Slovenská republika
                    
                        
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


    



    const adresaTPControlSettings = {
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
    
    Táto adresa pomôže nájsť najbližšie školy pre jednoduchšie dochádzanie.
    
                        
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


    



    const adresaZAControlSettings = {
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


                    
                        Späť
                        
                            Uložiť a odísť
                            Pridať dieťa
                            Uložiť
                        
                    
                &quot;) or . = concat(&quot;
                    
                        Krok 2/2
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
        
    
Slovenská republika

                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                    Trvalý pobyt

                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        Tomášikova 5723/13, 82101, Bratislava, Slovenská republika
                    
                        
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


    



    const adresaTPControlSettings = {
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
    
    Táto adresa pomôže nájsť najbližšie školy pre jednoduchšie dochádzanie.
    
                        
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


    



    const adresaZAControlSettings = {
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


                    
                        Späť
                        
                            Uložiť a odísť
                            Pridať dieťa
                            Uložiť
                        
                    
                &quot;))]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#step-2 > div.container-with-border</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[(text() = 'Slovenská republika' or . = 'Slovenská republika ')]</value>
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
      <webElementGuid>6901058a-9b0d-4e68-b73f-559911664e66</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container-with-border</value>
      <webElementGuid>8eccbde4-bf4c-46de-a592-451d7f05b252</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    
                        Krok 2/2
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
        
    
Slovenská republika

                    
                    
                        

    
        Materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    
                    
                        

    
        Iný materinský jazyk
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


                    

                    Trvalý pobyt

                    
                        

    
    
        Adresa trvalého pobytu dieťaťa
        (nepovinné)
    
    
    
    
                        
                        Tomášikova 5723/13, 82101, Bratislava, Slovenská republika
                    
                        
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


    



    const adresaTPControlSettings = {
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
    
    Táto adresa pomôže nájsť najbližšie školy pre jednoduchšie dochádzanie.
    
                        
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


    



    const adresaZAControlSettings = {
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


                    
                        Späť
                        
                            Uložiť a odísť
                            Pridať dieťa
                            Uložiť
                        
                    
                </value>
      <webElementGuid>0c3b7d94-14b5-4fd1-b317-71d881598c41</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;step-2&quot;)/div[@class=&quot;container-with-border&quot;]</value>
      <webElementGuid>e1985707-3b81-4143-8caa-dbb57fd6c180</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='step-2']/div</value>
      <webElementGuid>309c8d4c-c86b-49c9-8177-3c82fc128d27</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Národnosť'])[1]/following::div[3]</value>
      <webElementGuid>406e200a-7a4e-48c2-abfa-12c975035125</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Miesto narodenia'])[1]/following::div[6]</value>
      <webElementGuid>0e1b8289-3bb9-421f-a4a0-21e890c380aa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[4]/div[2]/div[2]/div</value>
      <webElementGuid>f1050c89-8b20-4d69-963c-2b9225564f8e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;&#xd;
                    &#xd;
                        Krok 2/2&#xd;
                        Štátna príslušnosť a jazyk&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Miesto narodenia&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Národnosť&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        slovenská&#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Štátna príslušnosť&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        Slovenská republika&#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
Slovenská republika&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Materinský jazyk&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Iný materinský jazyk&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
&#xd;
                    Trvalý pobyt&#xd;
&#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
    &#xd;
        Adresa trvalého pobytu dieťaťa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
    &#xd;
                        &#xd;
                        Tomášikova 5723/13, 82101, Bratislava, Slovenská republika&#xd;
                    &#xd;
                        &#xd;
                        Iná adresa trvalého pobytu&#xd;
                    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Krajina&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Obec&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Ulica&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Súpisné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
             &#xd;
            &#xd;
                /&#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Orientačné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        PSČ&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Adresa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
        warning&#xd;
        &#xd;
        &#xd;
            0/100&#xd;
        &#xd;
    &#xd;
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    const adresaTPControlSettings = {&#xd;
        adresaTPKrajina: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPObec: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPUlica: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPSupisneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte súpisné číslo.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{1,10}$/,&#xd;
                    message: &quot;Zadajte súpisné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPOrientacneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte orientačné číslo.&quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^.{1,20}$/,&#xd;
                    message: &quot;Zadajte orientačné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPPSC: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
               {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,&#xd;
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPAdresa: {&#xd;
            label: &quot;Adresa&quot;,&#xd;
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,&#xd;
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,&#xd;
                    message: &quot;Zadajte zahraničnú adresu.&quot;&#xd;
                }&#xd;
            ]&#xd;
        }&#xd;
    };&#xd;
&#xd;
&#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
    &#xd;
        Adresa miesta, kde dieťa zvyčajne žije&#xd;
        (nepovinné)&#xd;
    &#xd;
    Táto adresa pomôže nájsť najbližšie školy pre jednoduchšie dochádzanie.&#xd;
    &#xd;
                        &#xd;
                        Zhodná s adresou trvalého pobytu dieťaťa&#xd;
                    &#xd;
                        &#xd;
                        Iná adresa&#xd;
                    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Krajina&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Obec&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Ulica&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Súpisné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
             &#xd;
            &#xd;
                /&#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Orientačné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        PSČ&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Adresa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
        warning&#xd;
        &#xd;
        &#xd;
            0/100&#xd;
        &#xd;
    &#xd;
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    const adresaZAControlSettings = {&#xd;
        adresaZAKrajina: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAObec: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAUlica: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZASupisneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte súpisné číslo.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{1,10}$/,&#xd;
                    message: &quot;Zadajte súpisné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAOrientacneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte orientačné číslo.&quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^.{1,20}$/,&#xd;
                    message: &quot;Zadajte orientačné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAPSC: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
               {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,&#xd;
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAAdresa: {&#xd;
            label: &quot;Adresa&quot;,&#xd;
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,&#xd;
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,&#xd;
                    message: &quot;Zadajte zahraničnú adresu.&quot;&#xd;
                }&#xd;
            ]&#xd;
        }&#xd;
    };&#xd;
&#xd;
&#xd;
                    &#xd;
                        Späť&#xd;
                        &#xd;
                            Uložiť a odísť&#xd;
                            Pridať dieťa&#xd;
                            Uložiť&#xd;
                        &#xd;
                    &#xd;
                &quot;) or . = concat(&quot;&#xd;
                    &#xd;
                        Krok 2/2&#xd;
                        Štátna príslušnosť a jazyk&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Miesto narodenia&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Národnosť&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        slovenská&#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Štátna príslušnosť&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        Slovenská republika&#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
Slovenská republika&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Materinský jazyk&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
        Iný materinský jazyk&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
                    &#xd;
&#xd;
                    Trvalý pobyt&#xd;
&#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
    &#xd;
        Adresa trvalého pobytu dieťaťa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
    &#xd;
                        &#xd;
                        Tomášikova 5723/13, 82101, Bratislava, Slovenská republika&#xd;
                    &#xd;
                        &#xd;
                        Iná adresa trvalého pobytu&#xd;
                    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Krajina&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Obec&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Ulica&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Súpisné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
             &#xd;
            &#xd;
                /&#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Orientačné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        PSČ&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Adresa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
        warning&#xd;
        &#xd;
        &#xd;
            0/100&#xd;
        &#xd;
    &#xd;
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    const adresaTPControlSettings = {&#xd;
        adresaTPKrajina: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPObec: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPUlica: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPSupisneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte súpisné číslo.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{1,10}$/,&#xd;
                    message: &quot;Zadajte súpisné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPOrientacneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte orientačné číslo.&quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^.{1,20}$/,&#xd;
                    message: &quot;Zadajte orientačné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPPSC: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
               {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,&#xd;
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaTPAdresa: {&#xd;
            label: &quot;Adresa&quot;,&#xd;
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,&#xd;
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,&#xd;
                    message: &quot;Zadajte zahraničnú adresu.&quot;&#xd;
                }&#xd;
            ]&#xd;
        }&#xd;
    };&#xd;
&#xd;
&#xd;
                    &#xd;
                        &#xd;
&#xd;
    &#xd;
    &#xd;
        Adresa miesta, kde dieťa zvyčajne žije&#xd;
        (nepovinné)&#xd;
    &#xd;
    Táto adresa pomôže nájsť najbližšie školy pre jednoduchšie dochádzanie.&#xd;
    &#xd;
                        &#xd;
                        Zhodná s adresou trvalého pobytu dieťaťa&#xd;
                    &#xd;
                        &#xd;
                        Iná adresa&#xd;
                    &#xd;
    &#xd;
&#xd;
                    &#xd;
                    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Krajina&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Obec&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Ulica&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        &#xd;
        warning&#xd;
        &#xd;
            keyboard_arrow_down&#xd;
        &#xd;
    &#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Súpisné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
        &#xd;
             &#xd;
            &#xd;
                /&#xd;
            &#xd;
        &#xd;
        &#xd;
            &#xd;
&#xd;
    &#xd;
        Orientačné číslo&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
        &#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        PSČ&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
    &#xd;
        &#xd;
        warning&#xd;
        warning&#xd;
        &#xd;
            calendar_month&#xd;
        &#xd;
    &#xd;
    &#xd;
&#xd;
    &#xd;
&#xd;
    &#xd;
        &#xd;
&#xd;
    &#xd;
        Adresa&#xd;
        (nepovinné)&#xd;
    &#xd;
    &#xd;
        warning&#xd;
        &#xd;
        &#xd;
            0/100&#xd;
        &#xd;
    &#xd;
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&#xd;
&#xd;
&#xd;
    &#xd;
&#xd;
&#xd;
&#xd;
    const adresaZAControlSettings = {&#xd;
        adresaZAKrajina: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAObec: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Obec&quot; , &quot;'&quot; , &quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;255&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte obec po zadaní prvých 3 znakov a vyberte obec zo zoznamu zobrazených obcí!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAUlica: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Ulica&quot; , &quot;'&quot; , &quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Vyhľadajte ulicu po zadaní prvých 3 znakov a vyberte ulicu zo zoznamu zobrazených ulíc!&quot; , &quot;'&quot; , &quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZASupisneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Súpisné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte súpisné číslo.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{1,10}$/,&#xd;
                    message: &quot;Zadajte súpisné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAOrientacneCislo: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;Orientačné číslo&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;Zadajte orientačné číslo.&quot;,&#xd;
            required: false,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^.{1,20}$/,&#xd;
                    message: &quot;Zadajte orientačné číslo.&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAPSC: {&#xd;
            label: &quot; , &quot;'&quot; , &quot;PSČ&quot; , &quot;'&quot; , &quot;,&#xd;
            regexError: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
               {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[0-9]{3} ?[0-9]{2}$/,&#xd;
                    message: &quot;PSČ musí obsahovať 5 číslic (napr. 81101) alebo 3 číslice, medzeru a 2 číslice (napr. 811 01).&quot;&#xd;
                }&#xd;
            ]&#xd;
        },&#xd;
        adresaZAAdresa: {&#xd;
            label: &quot;Adresa&quot;,&#xd;
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,&#xd;
            regexError: &quot;Zadajte zahraničnú adresu.&quot;,&#xd;
            required: true,&#xd;
            attributes: {&#xd;
                maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot;&#xd;
            },&#xd;
            validators: [&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,&#xd;
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;&#xd;
                },&#xd;
                {&#xd;
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,&#xd;
                    functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,&#xd;
                    message: &quot;Zadajte zahraničnú adresu.&quot;&#xd;
                }&#xd;
            ]&#xd;
        }&#xd;
    };&#xd;
&#xd;
&#xd;
                    &#xd;
                        Späť&#xd;
                        &#xd;
                            Uložiť a odísť&#xd;
                            Pridať dieťa&#xd;
                            Uložiť&#xd;
                        &#xd;
                    &#xd;
                &quot;))]</value>
      <webElementGuid>2dc2143d-d682-4869-9fe5-8136e9f4702e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
