<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Odhlsi_govuk-wizard-steps</name>
   <tag></tag>
   <elementGuidId>f1c43949-f5d7-4c98-a375-470d013d6c5b</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.govuk-wizard-steps</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='prihlaska']/div/div/div/div</value>
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
      <webElementGuid>7977c3db-2f16-4ddc-b426-10b94bd6745e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>govuk-wizard-steps</value>
      <webElementGuid>b20a0259-bb22-4919-b3f6-e4f4485d4565</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    

    
                        
                            Prihlášky
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Prihlášky
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    check_circleVaše údaje sme automaticky uložili.close
                    
                    
                        
                            
                                Krok5/9
                             
                            
                                arrow_forward_ios
                                
                                    
                                
                                    1.
                                    Vybrať dieťa
                                
                            
                                
                                    2.
                                    Doplňujúce informácie o dieťati
                                
                            
                                
                                    3.
                                    Vybrať školy
                                
                            
                                
                                    4.
                                    Zákonný zástupca dieťaťa
                                
                            
                                
                                    5.
                                    Informácie o základnej škole
                                
                            
                                
                                    6.
                                    Výsledky vzdelávania na základnej škole
                                
                            
                                
                                    7.
                                    Súťaže
                                
                            
                                
                                    8.
                                    Pridať prílohy
                                
                            
                                
                                    9.
                                    Súhrnný prehľad
                                
                            
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    

                    
                        

    const pageTextsVyberDietata = {
        rozpracovana: '&lt;span class=&quot;gray-text&quot;>(Rozpracovan&amp;#xE1;)&lt;/span>',
        dietaLabel: 'Zvoľte osobu',
        povinneError: &quot;Toto pole je povinné.&quot;,
        '8049': 'Úspešne ste pridali údaje o dieťati.',
        dieta: 'dieťa',
        dietata: 'dieťaťa',
        dietati: 'dieťati',
        ziaka: 'žiaka',
        ziakovi: 'žiakovi',
        inyZiak: 'Iný žiak',
        ineDieta: 'Iné dieťa',
        pridajteDietaAleboOsobu: 'Pridajte dieťa alebo osobu vo vašej starostlivosti.',
    }



    Vybrať žiaka
    
        
            Vyberte osobu, za ktorú chcete podať prihlášku.
        

        
            
                Pridajte dieťa alebo osobu vo vašej starostlivosti.
                
                    Pridať žiaka
                
            

            
                Polia označené hviezdičkou sú povinné
                

    
    
        Zvoľte osobu
        (nepovinné)
    
    
    
    
                        
                        Lívia Bystrá (27.11.2009) 
                         
                        
                        Rudolf Tlačienka (12.11.2009) 
                         
                        
                        Júlia Oravská (26.9.2009) 
                         
                        
                        Iný žiak 
                         Pridajte dieťa alebo osobu vo vašej starostlivosti.  
    

                
                    
                        Pridať žiaka
                    
                
            
        
    
    
        
            Skontrolujte údaje o žiakovi
        

        
            
                
                    Osobné údaje žiaka
                    Upraviť
                
                
                    
                        Meno
                        Lívia
                    
                    
                        Priezvisko
                        Bystrá
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        091127/1130
                    
                    
                        Dátum narodenia
                        27.11.2009
                    
                    
                        Pohlavie
                        muž
                    
                    
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
                    
                    
                        Adresa trvalého pobytu
                        Na Rybníkoch 58/232, 01508, Jesenské (Levice), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu
                        
                    
                
            
        
    

    
        
        
    
    


                    
                    
                        

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
                    label: &quot;Iná&quot;,
                    value: 'Iná'
                }
            ],
            direction: 'column',
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            label: &quot;Typ&quot;,
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
            label: &quot;Záujem o školský klub detí&quot;,
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
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.&quot;,
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
        predprimarneVzdelavanieVJazyku: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: 'required',
                    message: 'Toto pole je povinné.'
                }
            ],
            required: true
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
            label: &quot;Má žiak zmenenú pracovnú schopnosť&quot;,
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
                         
                        
                        Iná 
                         
            
            

    
        Typ
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
                         
    

        

        
            

    
    
        Záujem o školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
        
            

    
    
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
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť
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
                         
    

            
        

        

        
            

    
        Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

        

        

    
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
        
    
    Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.


        

        

    
    
    
        
            
                Pokračovanie v plnení povinného predprimárneho vzdelávania
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            (nepovinné)
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
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

        vyberSkolyDescription: &quot;Vyberte školy, na ktoré chcete prihlásiť svoje dieťa. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSS: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory.&quot;,
        vyberSkolyDescriptionSSAV: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSSPostihnutie: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku.&quot;,
        vyberSkolyDescriptionSSPostihnutieAV: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku. Neskôr budete mať možnosť tieto odbory zoradiť podľa vašich preferencií.&quot;,       
        vyberSkolyDescriptionSkontrolujte: &quot;Skontrolujte školy, ktoré ste pridali do prihlášky. Neskôr ich zoradíte podľa vašich preferencií.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbory: &quot;Doplňte informácie k odborom a zvoľte ich poradie v prihláške.&quot;,
        vyberteSkolyPoradieDescriptionSSOdboryAutomatVypnuty: &quot;Skontrolujte odbory, ktoré ste pridali do prihlášky. Ak je zoznam odborov v prihláške kompletný, posuňte sa ďalej vo vypĺňaní prihlášky.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbor: &quot;Doplňte informácie k odboru.&quot;,
        
        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces. Zoraďte školy tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;odbor ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText2: &quot;odbory ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText5: &quot;odborov ste pridali do prihlášky.&quot;,

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



    Vybrať školy
    
        
            
                Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.
            
        

        
            

    const controlSettingsNajstSkolu = {
        typSelect: {
            label: &quot;Typ&quot;
        },
        vzdialenostSelect: {
            label: &quot;Vzdialenosť od zadanej adresy&quot;,
            showRequiredOrOptional: false
        },
        jazykySelect: {
            label: &quot;Jazyky&quot;
        },
        nazovOdboruSelect: {
            label: &quot;Názov odboru&quot;,
            searchPlaceholder: &quot;Vyhľadajte odbor&quot;,
        },
        lokalitaSelect: {
            label: &quot;Lokalita&quot;
        },
        kategoriaSkolySelect: {
            label: &quot;Kategória školy&quot;
        },
        zriadovatelSkolySelect: {
            label: &quot;Zriaďovateľ školy&quot;
        },
        vyucovaciJazykSelect: {
            label: &quot;Vyučovací jazyk&quot;
        },
        skupinaOdborovSelect: {
            label: &quot;Skupina odborov&quot;
        },
        ukoncenieStudiaSelect: {
            label: &quot;Ukončenie štúdia&quot;
        },
        dlzkaStudiaSelect: {
            label: &quot;Dĺžka štúdia&quot;
        },
        prijimaciaSkuskaSelect: {
            label: &quot;Prijímacia skúška&quot;,
            predmetPS: &quot;Má skúšku z&quot;,
            niePredmetPS: &quot;Nemá skúšku z&quot;,
        },
        nazovSkolyAleboAdresa: &quot;Názov školy alebo jej adresa&quot;,
        volnaKapacita: {
            label: &quot;Voľná kapacita na prijatie&quot;,
            showRequiredOrOptional: false,
            element: 'volnaKapacita',
        },
        talentovyOdborCB: {
            label: &quot;Talentový odbor&quot;,
            showRequiredOrOptional: false,
            element: 'talentovyOdbor',
        },
        dualneVzdelavanie:{
            label: &quot;Duálne vzdelávanie&quot;,
            showRequiredOrOptional: false,
            element: 'dualneVzdelavanie',
        },
        bezPrijimacejSkuskyCB: {
            label: &quot;Bez prijímacej skúšky&quot;,
            showRequiredOrOptional: false,
            element: 'bezPrijimacejSkusky',
        },
        skrytRozsirenyFilter: 'Skryť rozšírený filter',
        zobrazitRozsirenyFilter: 'Zobraziť rozšírený filter',
        oblubenaSkola: ' škola pridaná do obľúbených',
        oblubeneSkoly: '  školy pridané do obľúbených',
        oblubenychSkol: '  škôl pridaných do obľúbených',
        skolySteVybrali: ' školy ste pridali do prihlášky.',
        skoluSteVybrali: ' školu ste pridali do prihlášky.',
        skolSteVybrali: ' škôl ste pridali do prihlášky.',
        skolZodpovedaKriteriam: ' škôl zodpovedá vašim kritériám.',
        skolaZodpovedaKriteriam: ' škola zodpovedá vašim kritériám.',
        skolyZodpovedajuKriteriam: ' školy zodpovedajú vašim kritériám.',
        '8052': 'Váš zoznam vybraných škôl je prázdny. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.',
        loading: 'Načítava sa',
        z: 'z',
        oznacitVsetky: 'Označiť všetky',
        zobrazitVysledky: 'Zobraziť výsledky',

        nieSuDataOPoctePrijatych: 'Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.',
        nieSuDataOZaujme: 'Nie sú dostupné dáta o predbežnom záujme o odbor.',
        nieSuDataOUplatneni: 'Nie sú dostupné dáta o uplatnení uchádzačov.',
        nieSuData: 'Nie sú dostupné dáta.',

        talentovyOdbor: 'Talentový odbor',
        netalentovyOdbor: 'Netalentový odbor',
        nevykonavaSa: 'Nevykonáva sa',
        zobrazitZoznamZamestnavatelov: 'Zobraziť zoznam zamestnávateľov',
        bezPrijimacejSkusky: 'bez prijímacej skúšky',
        skolaSpolupracuje: 'Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.',
        kapacitaDual: 'Kapacita pre duálne vzdelávanie:',
        kapacitaDualHint: 'Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.',
        hladatPodlaMojejAdresyLabel: 'Hľadať podľa mojej adresy',
        hladatPodlaNazvuSkolyLabel: 'Hľadať podľa názvu školy alebo jej adresy',

        ms: 'Materské školy',
        zs: 'Základné školy',
        ss: 'Stredné školy',

    }



    
        
            
                
                    Materské školy
                
                
                    Základné školy
                
                
                    Stredné školy
                
                
                    Obľúbené školy
                    
                
                
                    Vybrané školy
                    1
                
            
        
    


    
        
        
        
        
        
            
                
                    
                        
                            

    Názov odboru
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            matematická analýzakeyboard_arrow_downkeyboard_arrow_uppaleontológia - paleontologia prekambrickych kralikovkeyboard_arrow_downkeyboard_arrow_upochrana životného prostredia a cirkulárna ekonomikakeyboard_arrow_downkeyboard_arrow_uptechnik mineralurgkeyboard_arrow_downkeyboard_arrow_upautotronikkeyboard_arrow_downkeyboard_arrow_upmechanik automobilových liniekkeyboard_arrow_downkeyboard_arrow_upinteligentné technológiekeyboard_arrow_downkeyboard_arrow_uppočítačové systémykeyboard_arrow_downkeyboard_arrow_upmultimédiákeyboard_arrow_downkeyboard_arrow_upinformačné a digitálne technológiekeyboard_arrow_downkeyboard_arrow_upsprávca inteligentných a digitálnych systémovkeyboard_arrow_downkeyboard_arrow_upprogramovanie digitálnych technológiíkeyboard_arrow_downkeyboard_arrow_upchemická informatikakeyboard_arrow_downkeyboard_arrow_uppotravinárstvo - spracúvanie mliekakeyboard_arrow_downkeyboard_arrow_upvýživa, ochrana zdravia a hodnotenie potravínkeyboard_arrow_downkeyboard_arrow_upoperátor potravinárskej výrobykeyboard_arrow_downkeyboard_arrow_uppracovník v potravinárstve - výroba trvanlivých potravínkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - výroba piva, sladu, nealkoholických nápojov a spracovanie minerálnych vôdkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - liehovarnícka výroba, výroba vína a nealkoholických nápojovkeyboard_arrow_downkeyboard_arrow_upoperátor textilnej výrobykeyboard_arrow_downkeyboard_arrow_uppraktická ženakeyboard_arrow_downkeyboard_arrow_upvýroba konfekciekeyboard_arrow_downkeyboard_arrow_upmurárkeyboard_arrow_downkeyboard_arrow_uptechnik vodár vodohospodárkeyboard_arrow_downkeyboard_arrow_upstavebná výroba - stavebné prácekeyboard_arrow_downkeyboard_arrow_upprevádzka a ekonomika dopravykeyboard_arrow_downkeyboard_arrow_uplodníkkeyboard_arrow_downkeyboard_arrow_upbezpečnosť a ochrana zdravia pri prácikeyboard_arrow_downkeyboard_arrow_upagromechanizátor, opravárkeyboard_arrow_downkeyboard_arrow_upagromechatronikkeyboard_arrow_downkeyboard_arrow_uppoľnohospodárska výroba - oprava poľnohospodárskych strojovkeyboard_arrow_downkeyboard_arrow_uplesná výrobakeyboard_arrow_downkeyboard_arrow_upekonomika a riadenie podnikov - Marketingkeyboard_arrow_downkeyboard_arrow_uphospodárska informatikakeyboard_arrow_downkeyboard_arrow_upbankový pracovníkkeyboard_arrow_downkeyboard_arrow_upmedzinárodné podnikaniekeyboard_arrow_downkeyboard_arrow_upškola podnikaniakeyboard_arrow_downkeyboard_arrow_upčašník, servírkakeyboard_arrow_downkeyboard_arrow_upobchodno-administratívny zamestnaneckeyboard_arrow_downkeyboard_arrow_upmarketingová komunikáciakeyboard_arrow_downkeyboard_arrow_upgymnáziumkeyboard_arrow_downkeyboard_arrow_upgymnázium - QSI International Programkeyboard_arrow_downkeyboard_arrow_upgymnázium - medzinárodný bakalársky program International Baccalaureate (IB) programkeyboard_arrow_downkeyboard_arrow_upšpeciálna základná škola 1. stupeň - I.cykluskeyboard_arrow_downkeyboard_arrow_uphudba - hra na husliach, viole, violončele, kontrabase, harfe, gitare, cimbalekeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie keramiky a porcelánukeyboard_arrow_downkeyboard_arrow_updigitálna maľba - koncept artkeyboard_arrow_downkeyboard_arrow_upfotografický dizajnkeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie skla-hutnícke tvarovanie skla - hutnícke tvarovanie sklakeyboard_arrow_downkeyboard_arrow_upumelecký parochniar a maskérkeyboard_arrow_downkeyboard_arrow_updizajn exteriérukeyboard_arrow_downkeyboard_arrow_upvojenská polícia - vojenská políciakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            Názov školy alebo jej adresa
                            
                                
                                
                                    search
                                
                            
                        
                        
                            

    
    
    
        
            
                Voľná kapacita na prijatie
                
            
            (nepovinné)
            
        
    

                        
                    
                    
                        
                            

    Lokalita
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Bratislavskýkeyboard_arrow_downkeyboard_arrow_upBratislavakeyboard_arrow_downkeyboard_arrow_upBratislava Ikeyboard_arrow_downkeyboard_arrow_upBratislava IIkeyboard_arrow_downkeyboard_arrow_upBratislava IIIkeyboard_arrow_downkeyboard_arrow_upBratislava IVkeyboard_arrow_downkeyboard_arrow_upBratislava Vkeyboard_arrow_downkeyboard_arrow_upMalackykeyboard_arrow_downkeyboard_arrow_upNitrianskykeyboard_arrow_downkeyboard_arrow_upKomárnokeyboard_arrow_downkeyboard_arrow_upŽilinskýkeyboard_arrow_downkeyboard_arrow_upŽilinakeyboard_arrow_downkeyboard_arrow_upBytčakeyboard_arrow_downkeyboard_arrow_upBanskobystrickýkeyboard_arrow_downkeyboard_arrow_upLučeneckeyboard_arrow_downkeyboard_arrow_upZvolenkeyboard_arrow_downkeyboard_arrow_upPrešovskýkeyboard_arrow_downkeyboard_arrow_upPrešovkeyboard_arrow_downkeyboard_arrow_upMedzilaborcekeyboard_arrow_downkeyboard_arrow_upSvidníkkeyboard_arrow_downkeyboard_arrow_upKošickýkeyboard_arrow_downkeyboard_arrow_upKošicekeyboard_arrow_downkeyboard_arrow_upKošice IIkeyboard_arrow_downkeyboard_arrow_upKošice - okoliekeyboard_arrow_downkeyboard_arrow_upSobrancekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Kategória školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Gymnáziumkeyboard_arrow_downkeyboard_arrow_upGymnáziumkeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upDopravná akadémiakeyboard_arrow_downkeyboard_arrow_upPedagogická školakeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upHotelová akadémiakeyboard_arrow_downkeyboard_arrow_upStredná priemyselná školakeyboard_arrow_downkeyboard_arrow_upObchodná akadémiakeyboard_arrow_downkeyboard_arrow_upOdborné učilištia špeciálne a praktické školykeyboard_arrow_downkeyboard_arrow_upTelesné postihnutiekeyboard_arrow_downkeyboard_arrow_upSluchové postihnutiekeyboard_arrow_downkeyboard_arrow_upZrakové postihnutiekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        


                        
                            

    Zriaďovateľ školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            cirkevnákeyboard_arrow_downkeyboard_arrow_upsúkromnákeyboard_arrow_downkeyboard_arrow_upštátnakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Vyučovací jazyk
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Výučba v jednom jazykukeyboard_arrow_downkeyboard_arrow_upSlovenskýkeyboard_arrow_downkeyboard_arrow_upBilingválna výučbakeyboard_arrow_downkeyboard_arrow_upČínskykeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_upS medzinárodným programomkeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                

                
                    
                        
                            

    Skupina odborov
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            prírodné vedykeyboard_arrow_downkeyboard_arrow_upfyzikálno-matematické vedykeyboard_arrow_downkeyboard_arrow_upgeologické vedykeyboard_arrow_downkeyboard_arrow_upochrana životného prostrediakeyboard_arrow_downkeyboard_arrow_uptechnické vedy a náukykeyboard_arrow_downkeyboard_arrow_upbaníctvo, geológia a geotechnikakeyboard_arrow_downkeyboard_arrow_upstrojárstvo a ostatná kovospracúvacia výroba IIkeyboard_arrow_downkeyboard_arrow_upinformatika a výpočtová technikakeyboard_arrow_downkeyboard_arrow_uptechnická a aplikovaná chémiakeyboard_arrow_downkeyboard_arrow_uppotravinárstvokeyboard_arrow_downkeyboard_arrow_uptextil a odevníctvokeyboard_arrow_downkeyboard_arrow_upstavebníctvo, geodézia a kartografiakeyboard_arrow_downkeyboard_arrow_updoprava, pošty a telekomunikáciekeyboard_arrow_downkeyboard_arrow_upšpeciálne technické odborykeyboard_arrow_downkeyboard_arrow_uppoľnohospodársko-lesnícke a veterinárne vedy a náukykeyboard_arrow_downkeyboard_arrow_uppoľnohospodárstvo, lesné hospodárstvo a rozvoj vidieka IIkeyboard_arrow_downkeyboard_arrow_upspoločenské vedy, náuky a službykeyboard_arrow_downkeyboard_arrow_upekonomické vedykeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby Ikeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby IIkeyboard_arrow_downkeyboard_arrow_uppublicistika, knihovníctvo a vedecké informáciekeyboard_arrow_downkeyboard_arrow_uppredprimárne, základné a úplné stredné vzdelanie na gymnáziukeyboard_arrow_downkeyboard_arrow_upvedy a náuky o kultúre a umeníkeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba Ikeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba IIkeyboard_arrow_downkeyboard_arrow_upUmenie a umeleckoremeselná tvorba  IIIkeyboard_arrow_downkeyboard_arrow_upvojenské a bezpečnostné vedy a náukykeyboard_arrow_downkeyboard_arrow_upbezpečnostné službykeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Ukončenie štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            maturitná skúškakeyboard_arrow_downkeyboard_arrow_upmaturitná skúška a výučný listkeyboard_arrow_downkeyboard_arrow_upvýučný listkeyboard_arrow_downkeyboard_arrow_upvyučenie a praktické školykeyboard_arrow_downkeyboard_arrow_upodborná prax alebo výcvikkeyboard_arrow_downkeyboard_arrow_upinékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Dĺžka štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            6,5 ročnékeyboard_arrow_downkeyboard_arrow_up9 ročnékeyboard_arrow_downkeyboard_arrow_up1 ročnékeyboard_arrow_downkeyboard_arrow_up4,5 ročnékeyboard_arrow_downkeyboard_arrow_up3 ročnékeyboard_arrow_downkeyboard_arrow_up2 ročnékeyboard_arrow_downkeyboard_arrow_up5,5 ročnékeyboard_arrow_downkeyboard_arrow_up4 ročnékeyboard_arrow_downkeyboard_arrow_up6 ročnékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Prijímacia skúška
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Má skúšku zkeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upNemá skúšku zkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upRuský jazykkeyboard_arrow_downkeyboard_arrow_upNenachádza sa v číselníkukeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                    
                        
                            

    
    
    
        
            
                Talentový odbor
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Duálne vzdelávanie
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Bez prijímacej skúšky
                
            
            (nepovinné)
            
        
    

                        
                    
                
            

            
                
                    
                        Zobraziť rozšírený filter
                    
                    
                        keyboard_arrow_down
                    
                
                
                    
                        Skryť rozšírený filter
                    
                    
                        keyboard_arrow_up
                    
                
            
            
                Viac filtrov
                Menej filtrov
            

            
                Filtre
                
                    
                
                
                    Vymazať filtre
                
            
        

        
            +−

            
                
                
                    
                    
                    
                        
                            Stredné školy
                        
                        
                            1
                             škola zodpovedá vašim kritériám.
                        
                        
                            
                                
                                    
                                        
                                            filter_alt
                                        
                                        
                                            Zadajte kritéria vyhľadávania
                                        
                                    
                                    
                                        
                                            Zadajte kritéria vo vyhľadávaní, aby ste našli správne školy.
                                        
                                    
                                
                            
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        Pridať do obľúbených
                                                        Odstrániť z obľúbených
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            
                        
                            
                                1-1 z 1
                                
                                    chevron_left
                                    Predchádzajúca
                                
                                
                                    Ďalšia
                                    chevron_right
                                
                            
                        

                        
                        

                        

                        

                    

                    
                        
                            Obľúbené školy
                        
                        
                            0 obľúbených škôl
                            
                            
                        
                        
                        
                        
                            
                                
                                    
                                        bookmark_border
                                    
                                    
Žiadne obľúbené školy                                    
                                
                                
                                    
Zatiaľ nemáte obľúbené školy. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.                                    
                                
                            
                        
                    

                    
                        Stredné školy
                        
                            1
                            odbor ste pridali do prihlášky.
                        
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            

                        

                    
                
            
        
        
        
        
        


        
            
                
                    
                        
                            
                            
                        
                        
                            close
                        
                    
                    
                        
                            +−
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
        
            
                
                    
                        
                            Zoznam zamestnávateľov
                            
                        
                        
                            close
                        
                    
                    
                        
                            Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.
                            
                                Kapacita pre duálne vzdelávanie: 
                                Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.
                            
                        
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
    






        
    

    
        
            
                Doplňte informácie k odboru.
            
        

        
        

        
            Stredné školy
            
                1
                školu ste pridali do prihlášky.
            
        

        
            
                1
                Vami zvolené poradie školy
                
                    arrow_upward
                    Presunúť vyššie
                
                
                    Presunúť nižšie
                    arrow_downward
                
                drag_indicator
            
            
                
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
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            (nepovinné)
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                    
                    
                        

    
    
    
        
            
                Záujem o školský internát
                
            
            (nepovinné)
            
        
    

                    
                    
                        

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)1. termín (2.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

                Odstrániť z prihlášky
            

        
    

    

    

    


                    
                    
                        


    const pageTextsZakonnyZastupca = {
        '8042': 'Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.',
        dietata: &quot;dieťaťa&quot;,
        ziaka: &quot;žiaka&quot;
    };

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,

        zastupca1RodnePriezvisko: {
            label: 'Rodné priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },

        zastupca1Telefon: {
            label: 'Telefónne číslo',
            hint: 'Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX',
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: '+421',
            required: true
        },

        zastupca1AdresaRadio: {
            label: 'Uveďte adresu, na ktorú prijímate poštové zásielky',
            options: [
                {
                    label: 'window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana',
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: 'column',
        },

        zastupca2Radio: {
            label: 'Vyberte jednu z možností',
            options: [
                {
                    label: &quot;rfo&quot;,
                    value: &quot;RFO&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Iný zákonný zástupca&lt;/div>&lt;div class='govuk-hint'>Vyplňte v prípade, ak druhým zákonným zástupcom je iná osoba.&lt;/div>&lt;/div>&quot;,
                    value: &quot;INY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca je známy&lt;/div>&lt;div class='govuk-hint'>Vyplňte údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;ZNAMY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca nie je známy&lt;/div>&lt;div class='govuk-hint'>Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;NEZNAMY&quot;
                }
            ],
            direction: 'column',
        },

        zastupca2Meno: {
            label: 'Meno',
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2Priezvisko: {
            label: 'Priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2RodnePriezvisko: {
            label: 'Rodné priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        zastupca2RodneCislo: {
            label: 'Rodné číslo',
            hint: 'Zadajte vo formáte XXXXXX/XXXX',
            required: true
        },
        zastupca2NemaRodneCislo: {
            label: 'Osoba nemá pridelené rodné číslo',
            required: false
        },
        zastupca2DatumNarodenia: {
            label: 'Dátum narodenia',
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        zastupca2Email: {
            label: 'E-mail',
            hint: 'Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.',
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        zastupca2Telefon: {
            label: 'Telefónne číslo',
            hint: 'Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX',
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: '+421',
            required: true
        },
        zastupca2AdresaRadio: {
            label: 'Uveďte adresu, na ktorú prijímate poštové zásielky',
            options: [
                {
                    label: 'window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana',
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: 'column',
        },

        // rfo zastupca 2
        rfoZastupca2Meno: {
            label: 'Meno',
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2Priezvisko: {
            label: 'Priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2RodnePriezvisko: {
            label: 'Rodné priezvisko',
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        rfoZastupca2RodneCislo: {
            label: 'Rodné číslo',
            hint: 'Zadajte vo formáte XXXXXX/XXXX',
            required: true
        },
        rfoZastupca2DatumNarodenia: {
            label: 'Dátum narodenia',
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        rfoZastupca2Email: {
            label: 'E-mail',
            hint: 'Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.',
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        rfoZastupca2Telefon: {
            label: 'Telefónne číslo',
            hint: 'Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX',
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: '+421',
            required: true
        },
        rfoZastupca2AdresaRadio: {
            label: 'Uveďte adresu, na ktorú prijímate poštové zásielky',
            options: [
                {
                    label: 'window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana',
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: 'column',
        },

        agreementRadio: {
            label: 'Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca {osoba}.',
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

        komunikaciaLenSZZ1: {
            label: 'Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.',
            hint: 'Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).',
            required: false
        },
        noAgreementReason: {
            label: 'Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: ',
            required: true
        },

        DataCheckWarningBox: {
            headerText: 'Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty',
            contentSelector: '#dataCheckWarningBoxContent'
        }

    };





    
        Zákonný zástupca žiaka
        
            
                Skontrolujte a doplňte údaje o zákonných zástupcoch. V prípade potreby opravte nesprávne informácie.
            
        
    

    Polia označené hviezdičkou sú povinné

    
        
            Osobné údaje zákonného zástupcu č.1
        
        
            
                
                    Údaje z vášho profilu
                
                Upraviť
            
            
                
                    Meno
                    Stanislav
                
                
                    Priezvisko
                    Štefánek
                
                
                    Rodné priezvisko
                    -
                
                
                    Rodné číslo
                    030724/6027
                
                
                    Dátum narodenia
                    24.07.2003
                
                
                    Kontaktný e-mail
                    barcik@ditec.sk
                
            
        

        
            

    
        Rodné priezvisko
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
                        Iná korešpondenčná adresa 
                         
                

    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        

    
        Adresa
        (nepovinné)
    

    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.

    
        
    


    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window['zastupca1InaAdresaControlSettings'] = {
    zastupca1InaAdresaKrajina: {
        label: 'Krajina',
            required: true,
                attributes: { maxLength: '100' },
        validators: [
            { type: 'required', message: 'Toto pole je povinné.' },
            { type: 'select', message: 'Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!' }
        ]
    },

    zastupca1InaAdresaAdresaRA: {
        label: 'Adresa',
            hint: 'Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.',
                required: true,
                    placeholder: '',
                        validators: [
                            { type: 'required', message: 'Toto pole je povinné.' },
                            // ak user niečo napíše, musí to aj vybrať zo zoznamu
                            { type: 'select', message: '' }
                        ]
    },

    zastupca1InaAdresaAdresa: {
        label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
                required: true,
                    attributes: { maxLength: '100' },
        validators: [
            { type: 'required', message: 'Toto pole je povinné.' },
            {
                type: 'regex',
                functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\'\-\s]{1,100}$/,
                message: &quot;Zadajte zahraničnú adresu.&quot;
            }
        ]
    }
        };


            
    

            
            
        
    

    

    


        
            
                Osobné údaje zákonného zástupcu č.2
            
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známyVyplňte údaje druhého zákonného zástupcu. 
                         
            
                

    
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
            
        
    




    window['zastupca2RodneCisloControlSettings'] = {
        zastupca2RodneCislo: {
            label: 'Rodné číslo',
            hint: 'Zadajte vo formáte XXXXXX/XXXX',
            attributes: {
                 minLength: '10',
                 maxLength: '11',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: 'Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).'
                },
            ],

            validationMessageDelitelnost11: 'Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.',
            validationMessage9Miestne: '9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.',
            validationMessageTretiZnak: 'Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.',

        },
    };





            

    
    
    
        
            
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


            
    

            
            
        
                        
                        Druhý zákonný zástupca nie je známyTúto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu. 
                         
    

            
            
                

    
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
            
        
    




    window['rfoZastupca2RodneCisloControlSettings'] = {
        rfoZastupca2RodneCislo: {
            label: 'Rodné číslo',
            hint: 'Zadajte vo formáte XXXXXX/XXXX',
            attributes: {
                 minLength: '10',
                 maxLength: '11',
            },
            validators: [
                {
                    type: 'regex',
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: 'Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).'
                },
            ],

            validationMessageDelitelnost11: 'Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.',
            validationMessage9Miestne: '9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.',
            validationMessageTretiZnak: 'Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.',

        },
    };





            

    
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
    



    window['rfoZastupca2DatumNarodeniaControlSettings'] = {
        rfoZastupca2DatumNarodeniaDen: {
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
        rfoZastupca2DatumNarodeniaMesiac: {
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
        rfoZastupca2DatumNarodeniaRok: {
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


    



    window['rfoZastupca2AdresaControlSettings'] = {
        rfoZastupca2AdresaKrajina: {
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
        rfoZastupca2AdresaObec: {
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
        rfoZastupca2AdresaUlica: {
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
        rfoZastupca2AdresaUlicaReq: {
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
        rfoZastupca2AdresaSupisneCislo: {
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
        rfoZastupca2AdresaOrientacneCislo: {
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
        rfoZastupca2AdresaPSC: {
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
        rfoZastupca2AdresaAdresa: {
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


            
    

            
            
        
        

        
            error

    Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty
    
        
            
            
            
                rodný list dieťaťa (na nahliadnutie),
                úmrtný list druhého rodiča (na nahliadnutie),
                neoverenú kópiu súdneho rozhodnutia (môžete ju nahrať ako prílohu v nasledujúcich krokoch alebo doniesť na zápis).
            
        
        
        
            
            
        
    


        

        

        

        
    




    Zákonný zástupca žiaka
    
        
            Súhlas druhého zákonného zástupcu s podaním prihlášky.
        
    

    
        info
        
            Podľa zákona č. 245/2008 Z. z. o výchove a vzdelávaní (školský zákon) a o zmene a doplnení niektorých zákonov v znení neskorších predpisov sa na prihláške o prijatie vyžaduje podpis obidvoch zákonných zástupcov dieťaťa. Ak nie je možné získať podpis druhého zákonného zástupcu, je potrebné uviesť dôvod a priložiť potrebný dokument ako prílohu.Viac informácií
        
    

    Polia označené hviezdičkou sú povinné

    
        
            

    
    
        Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca žiaka.
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
            

    
    
    
        
            
                Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.
                Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
            
            (nepovinné)
            Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
        
    

        
                        
                        nie 
                         
    

        
            
                

    
        Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: 
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
            
                error
                
                    Doručenie prílohy je zákonnou povinnosťou. Zatiaľ ju nemusíte priložiť – vyžiadame ju neskôr
                    
                        Rozhodnutie súdu — ak bol druhému zákonnému zástupcovi obmedzený alebo pozastavený výkon rodičovských práv a povinností, je potrebné priložiť kópiu tohto rozhodnutia.
                        Potvrdenie od lekára — ak druhý zákonný zástupca nie je schopný podpísať sa zo zdravotných dôvodov, je toto potvrdenie potrebné.
                        Čestné vyhlásenie zákonného zástupcu — ak získanie podpisu druhého zákonného zástupcu je spojené s ťažko prekonateľnou prekážkou a je to v najlepšom záujme dieťaťa.
                    
                
            

        

        
        
    


                    
                    
                        

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
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
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
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte informácie o základnej škole, ktorú žiak aktuálne navštevuje.
        
    

    
        
            
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
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Zo školy v zahraničí 
                         
    

        

        

        

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        poľský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
    

    
        
            
                
                    
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
        zmeniliSteUdajePanelTitle: &quot;Zmenili ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        zmeniliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť nasledovné:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;Neuviedli ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        neuviedliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť overené kópie vysvedčení.&quot;,
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
            label: &quot;Mám známky v inej známkovacej schéme&quot;,
            hint: &quot;Označte, ak vaše známky nie sú v bežnej slovenskej škále (1 – 5 alebo výborný, chválitebný, dobrý, dostatočný, nedostatočný).&quot;,
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

    
        Za posledný ročník uveďte známky z polročného vysvedčenia. Ak žiak opakoval ročník, uveďte známky iba za ten ročník, v ktorom prospel.
        Skontrolujte, či všetky známky z požadovaných predmetov za posledné 4 ročníky základnej školy zodpovedajú údajom na vysvedčeniach. Ak žiak niektorý ročník opakoval, zapíšte známky z posledného absolvovaného ročníka. Chýbajúce alebo nesprávne známky  doplňte alebo opravte.
    

    
        Ak používate EduPage, môžete si známky jednoducho načítať priamo odtiaľ.
        Pridať známky z EduPage
    

    
        Známky v inej známkovacej schéme
        
            

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
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

    
        Uveďte iba súťaže, v ktorých žiak dosiahol významné umiestnenie alebo výsledok. Súťaže, za ktoré sa prideľujú body pri prijímacom konaní, nájdete v kritériách školy, na ktorú sa žiak hlási.
    

    
        
            
                
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
        spravaRozhodnutie: &quot;Rozhodnutie:&quot;,
        spravaDoplnenePodklady: &quot;Doplnené podklady:&quot;,
        spravaRiaditel: &quot;Riaditeľ&quot;,
        spravaDovod: &quot;Dôvod:&quot;,
        spravaPotvrditNastup: &quot;Potvrdiť nástup&quot;,
        spravaPridatPrilohu: &quot;Pridať prílohu&quot;,
        spravaPrilozeneDokumenty: &quot;Priložené dokumenty:&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        nenastupi: &quot;nenastúpi&quot;,
        identifikator: &quot;Identifikátor&quot;,
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
        stavPrihlasky: &quot;Stav prihlášky&quot;,
        podana: &quot;Podaná&quot;,
        zastupcaSuhlas: &quot;Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}&quot;,
        elektronicky: &quot;Elektronicky&quot;,
        prichodZoZahraniciaPDF: &quot;Zo zahraničia&quot;,
        ziadneSutaze: &quot;Neboli nahrané žiadne súťaže.&quot;,

        radoveCislovky:
        { 
            &quot;1&quot;: &quot;Prvý&quot;,
            &quot;2&quot;: &quot;Druhý&quot;,
            &quot;3&quot;: &quot;Tretí&quot;,
            &quot;4&quot;: &quot;Štvrtý&quot;,
            &quot;5&quot;: &quot;Piaty&quot;,
            &quot;6&quot;: &quot;Šiesty&quot;,
            &quot;7&quot;: &quot;Siedmy&quot;,
            &quot;8&quot;: &quot;Ôsmy&quot;,
            &quot;9&quot;: &quot;Deviaty&quot;,
            &quot;10&quot;: &quot;Desiaty&quot;
        },

        cestnePrehlasenie: {
            label: &quot;Čestné vyhlásenie&quot;,
            hint: &quot;Čestne vyhlasujem, že zákonný zástupca/zákonní zástupcovia a žiak, potvrdzujú správnosť a pravdivosť údajov v prihláške.&quot;,
            required: true,
            showRequiredHint: true,
        },
        suhlasOsobneUdaje: {
            label: &quot;Súhlas so spracúvaním osobných údajov&quot;,
            hint: &quot;&lt;div>Súhlasím so spracúvaním osobných údajov v rozsahu údajov uvedených v prihláške na účel podania prihlášky. &lt;a href='/Ochrana-osobnych-udajov'>Viac informácií nájdete tu.&lt;/a>&lt;/div>&quot;,
            required: true,
            showRequiredHint: true,
        },
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href='https://www.slovensko.sk/'>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }
    };




    Súhrnný prehľad
    
        
            Dôkladne skontrolujte všetky informácie. Urýchlite tak prijímací proces.
        
    

    Polia označené hviezdičkou sú povinné

    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
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
        
        
            Neboli nahrané žiadne súťaže.
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            

            
        
    

    
        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    


        
            
                
                    
                
                
                    Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                    

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

                
            
        
    

    
    




    Detail prihlášky


    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Kolo prijímacieho konania
                -
            
            
                Prístupový kód
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
        
        
            
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
            
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
        
        
            Osobné údaje zákonného zástupcu č. 1
            
                
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
                Informácie o základnej škole
            
        
        
            
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
            
        
        
        
    

    
        
            
                Súťaže
            
        
        
        
    

    

    
        
            
                Prílohy
            
        
        
            
            
        
    
    
    


EXPORT PDF
                    

                    
                        Späť
                        
                            Uložiť a odísť
                            Ďalej
                            Odoslať prihlášku
                                Stiahnuť XML
                            
                                delete_outline
                                Odstrániť
                            
                            Aktualizovať a odísť
                        
                    
                </value>
      <webElementGuid>462b2e06-39c0-418b-af83-13088494b1a7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;prihlaska&quot;)/div[@class=&quot;menu-layout&quot;]/div[@class=&quot;privatna-zona-content&quot;]/div[@class=&quot;govuk-form-group&quot;]/div[@class=&quot;govuk-wizard-steps&quot;]</value>
      <webElementGuid>efc8d408-67ea-425e-b59e-5abc69582e1f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='prihlaska']/div/div/div/div</value>
      <webElementGuid>df0aa5b1-ce2d-4c20-be58-e02ee0f0a246</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Odhlásiť'])[2]/following::div[5]</value>
      <webElementGuid>4d89818d-5570-4bae-8fba-24586b4ee056</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Môj profil'])[2]/following::div[5]</value>
      <webElementGuid>a628e767-e2c6-401b-96fb-9fa0d5b0147e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div/div/div</value>
      <webElementGuid>f4d255cc-ebbd-4938-acce-7ac6d98142e4</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                    

    
                        
                            Prihlášky
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Prihlášky
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    check_circleVaše údaje sme automaticky uložili.close
                    
                    
                        
                            
                                Krok5/9
                             
                            
                                arrow_forward_ios
                                
                                    
                                
                                    1.
                                    Vybrať dieťa
                                
                            
                                
                                    2.
                                    Doplňujúce informácie o dieťati
                                
                            
                                
                                    3.
                                    Vybrať školy
                                
                            
                                
                                    4.
                                    Zákonný zástupca dieťaťa
                                
                            
                                
                                    5.
                                    Informácie o základnej škole
                                
                            
                                
                                    6.
                                    Výsledky vzdelávania na základnej škole
                                
                            
                                
                                    7.
                                    Súťaže
                                
                            
                                
                                    8.
                                    Pridať prílohy
                                
                            
                                
                                    9.
                                    Súhrnný prehľad
                                
                            
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    

                    
                        

    const pageTextsVyberDietata = {
        rozpracovana: &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;gray-text&quot;>(Rozpracovan&amp;#xE1;)&lt;/span>&quot; , &quot;'&quot; , &quot;,
        dietaLabel: &quot; , &quot;'&quot; , &quot;Zvoľte osobu&quot; , &quot;'&quot; , &quot;,
        povinneError: &quot;Toto pole je povinné.&quot;,
        &quot; , &quot;'&quot; , &quot;8049&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Úspešne ste pridali údaje o dieťati.&quot; , &quot;'&quot; , &quot;,
        dieta: &quot; , &quot;'&quot; , &quot;dieťa&quot; , &quot;'&quot; , &quot;,
        dietata: &quot; , &quot;'&quot; , &quot;dieťaťa&quot; , &quot;'&quot; , &quot;,
        dietati: &quot; , &quot;'&quot; , &quot;dieťati&quot; , &quot;'&quot; , &quot;,
        ziaka: &quot; , &quot;'&quot; , &quot;žiaka&quot; , &quot;'&quot; , &quot;,
        ziakovi: &quot; , &quot;'&quot; , &quot;žiakovi&quot; , &quot;'&quot; , &quot;,
        inyZiak: &quot; , &quot;'&quot; , &quot;Iný žiak&quot; , &quot;'&quot; , &quot;,
        ineDieta: &quot; , &quot;'&quot; , &quot;Iné dieťa&quot; , &quot;'&quot; , &quot;,
        pridajteDietaAleboOsobu: &quot; , &quot;'&quot; , &quot;Pridajte dieťa alebo osobu vo vašej starostlivosti.&quot; , &quot;'&quot; , &quot;,
    }



    Vybrať žiaka
    
        
            Vyberte osobu, za ktorú chcete podať prihlášku.
        

        
            
                Pridajte dieťa alebo osobu vo vašej starostlivosti.
                
                    Pridať žiaka
                
            

            
                Polia označené hviezdičkou sú povinné
                

    
    
        Zvoľte osobu
        (nepovinné)
    
    
    
    
                        
                        Lívia Bystrá (27.11.2009) 
                         
                        
                        Rudolf Tlačienka (12.11.2009) 
                         
                        
                        Júlia Oravská (26.9.2009) 
                         
                        
                        Iný žiak 
                         Pridajte dieťa alebo osobu vo vašej starostlivosti.  
    

                
                    
                        Pridať žiaka
                    
                
            
        
    
    
        
            Skontrolujte údaje o žiakovi
        

        
            
                
                    Osobné údaje žiaka
                    Upraviť
                
                
                    
                        Meno
                        Lívia
                    
                    
                        Priezvisko
                        Bystrá
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        091127/1130
                    
                    
                        Dátum narodenia
                        27.11.2009
                    
                    
                        Pohlavie
                        muž
                    
                    
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
                    
                    
                        Adresa trvalého pobytu
                        Na Rybníkoch 58/232, 01508, Jesenské (Levice), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu
                        
                    
                
            
        
    

    
        
        
    
    


                    
                    
                        

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
                    label: &quot;Iná&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Iná&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            label: &quot;Typ&quot;,
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
            label: &quot;Záujem o školský klub detí&quot;,
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
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.&quot;,
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
        predprimarneVzdelavanieVJazyku: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
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
            label: &quot;Má žiak zmenenú pracovnú schopnosť&quot;,
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
                         
                        
                        Iná 
                         
            
            

    
        Typ
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
                         
    

        

        
            

    
    
        Záujem o školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
        
            

    
    
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
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť
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
                         
    

            
        

        

        
            

    
        Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

        

        

    
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
        
    
    Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.


        

        

    
    
    
        
            
                Pokračovanie v plnení povinného predprimárneho vzdelávania
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            (nepovinné)
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
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

        vyberSkolyDescription: &quot;Vyberte školy, na ktoré chcete prihlásiť svoje dieťa. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSS: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory.&quot;,
        vyberSkolyDescriptionSSAV: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSSPostihnutie: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku.&quot;,
        vyberSkolyDescriptionSSPostihnutieAV: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku. Neskôr budete mať možnosť tieto odbory zoradiť podľa vašich preferencií.&quot;,       
        vyberSkolyDescriptionSkontrolujte: &quot;Skontrolujte školy, ktoré ste pridali do prihlášky. Neskôr ich zoradíte podľa vašich preferencií.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbory: &quot;Doplňte informácie k odborom a zvoľte ich poradie v prihláške.&quot;,
        vyberteSkolyPoradieDescriptionSSOdboryAutomatVypnuty: &quot;Skontrolujte odbory, ktoré ste pridali do prihlášky. Ak je zoznam odborov v prihláške kompletný, posuňte sa ďalej vo vypĺňaní prihlášky.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbor: &quot;Doplňte informácie k odboru.&quot;,
        
        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces. Zoraďte školy tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;odbor ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText2: &quot;odbory ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText5: &quot;odborov ste pridali do prihlášky.&quot;,

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



    Vybrať školy
    
        
            
                Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.
            
        

        
            

    const controlSettingsNajstSkolu = {
        typSelect: {
            label: &quot;Typ&quot;
        },
        vzdialenostSelect: {
            label: &quot;Vzdialenosť od zadanej adresy&quot;,
            showRequiredOrOptional: false
        },
        jazykySelect: {
            label: &quot;Jazyky&quot;
        },
        nazovOdboruSelect: {
            label: &quot;Názov odboru&quot;,
            searchPlaceholder: &quot;Vyhľadajte odbor&quot;,
        },
        lokalitaSelect: {
            label: &quot;Lokalita&quot;
        },
        kategoriaSkolySelect: {
            label: &quot;Kategória školy&quot;
        },
        zriadovatelSkolySelect: {
            label: &quot;Zriaďovateľ školy&quot;
        },
        vyucovaciJazykSelect: {
            label: &quot;Vyučovací jazyk&quot;
        },
        skupinaOdborovSelect: {
            label: &quot;Skupina odborov&quot;
        },
        ukoncenieStudiaSelect: {
            label: &quot;Ukončenie štúdia&quot;
        },
        dlzkaStudiaSelect: {
            label: &quot;Dĺžka štúdia&quot;
        },
        prijimaciaSkuskaSelect: {
            label: &quot;Prijímacia skúška&quot;,
            predmetPS: &quot;Má skúšku z&quot;,
            niePredmetPS: &quot;Nemá skúšku z&quot;,
        },
        nazovSkolyAleboAdresa: &quot;Názov školy alebo jej adresa&quot;,
        volnaKapacita: {
            label: &quot;Voľná kapacita na prijatie&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;volnaKapacita&quot; , &quot;'&quot; , &quot;,
        },
        talentovyOdborCB: {
            label: &quot;Talentový odbor&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;talentovyOdbor&quot; , &quot;'&quot; , &quot;,
        },
        dualneVzdelavanie:{
            label: &quot;Duálne vzdelávanie&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;dualneVzdelavanie&quot; , &quot;'&quot; , &quot;,
        },
        bezPrijimacejSkuskyCB: {
            label: &quot;Bez prijímacej skúšky&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;bezPrijimacejSkusky&quot; , &quot;'&quot; , &quot;,
        },
        skrytRozsirenyFilter: &quot; , &quot;'&quot; , &quot;Skryť rozšírený filter&quot; , &quot;'&quot; , &quot;,
        zobrazitRozsirenyFilter: &quot; , &quot;'&quot; , &quot;Zobraziť rozšírený filter&quot; , &quot;'&quot; , &quot;,
        oblubenaSkola: &quot; , &quot;'&quot; , &quot; škola pridaná do obľúbených&quot; , &quot;'&quot; , &quot;,
        oblubeneSkoly: &quot; , &quot;'&quot; , &quot;  školy pridané do obľúbených&quot; , &quot;'&quot; , &quot;,
        oblubenychSkol: &quot; , &quot;'&quot; , &quot;  škôl pridaných do obľúbených&quot; , &quot;'&quot; , &quot;,
        skolySteVybrali: &quot; , &quot;'&quot; , &quot; školy ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skoluSteVybrali: &quot; , &quot;'&quot; , &quot; školu ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skolSteVybrali: &quot; , &quot;'&quot; , &quot; škôl ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skolZodpovedaKriteriam: &quot; , &quot;'&quot; , &quot; škôl zodpovedá vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        skolaZodpovedaKriteriam: &quot; , &quot;'&quot; , &quot; škola zodpovedá vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        skolyZodpovedajuKriteriam: &quot; , &quot;'&quot; , &quot; školy zodpovedajú vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8052&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Váš zoznam vybraných škôl je prázdny. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.&quot; , &quot;'&quot; , &quot;,
        loading: &quot; , &quot;'&quot; , &quot;Načítava sa&quot; , &quot;'&quot; , &quot;,
        z: &quot; , &quot;'&quot; , &quot;z&quot; , &quot;'&quot; , &quot;,
        oznacitVsetky: &quot; , &quot;'&quot; , &quot;Označiť všetky&quot; , &quot;'&quot; , &quot;,
        zobrazitVysledky: &quot; , &quot;'&quot; , &quot;Zobraziť výsledky&quot; , &quot;'&quot; , &quot;,

        nieSuDataOPoctePrijatych: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.&quot; , &quot;'&quot; , &quot;,
        nieSuDataOZaujme: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o predbežnom záujme o odbor.&quot; , &quot;'&quot; , &quot;,
        nieSuDataOUplatneni: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o uplatnení uchádzačov.&quot; , &quot;'&quot; , &quot;,
        nieSuData: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta.&quot; , &quot;'&quot; , &quot;,

        talentovyOdbor: &quot; , &quot;'&quot; , &quot;Talentový odbor&quot; , &quot;'&quot; , &quot;,
        netalentovyOdbor: &quot; , &quot;'&quot; , &quot;Netalentový odbor&quot; , &quot;'&quot; , &quot;,
        nevykonavaSa: &quot; , &quot;'&quot; , &quot;Nevykonáva sa&quot; , &quot;'&quot; , &quot;,
        zobrazitZoznamZamestnavatelov: &quot; , &quot;'&quot; , &quot;Zobraziť zoznam zamestnávateľov&quot; , &quot;'&quot; , &quot;,
        bezPrijimacejSkusky: &quot; , &quot;'&quot; , &quot;bez prijímacej skúšky&quot; , &quot;'&quot; , &quot;,
        skolaSpolupracuje: &quot; , &quot;'&quot; , &quot;Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.&quot; , &quot;'&quot; , &quot;,
        kapacitaDual: &quot; , &quot;'&quot; , &quot;Kapacita pre duálne vzdelávanie:&quot; , &quot;'&quot; , &quot;,
        kapacitaDualHint: &quot; , &quot;'&quot; , &quot;Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.&quot; , &quot;'&quot; , &quot;,
        hladatPodlaMojejAdresyLabel: &quot; , &quot;'&quot; , &quot;Hľadať podľa mojej adresy&quot; , &quot;'&quot; , &quot;,
        hladatPodlaNazvuSkolyLabel: &quot; , &quot;'&quot; , &quot;Hľadať podľa názvu školy alebo jej adresy&quot; , &quot;'&quot; , &quot;,

        ms: &quot; , &quot;'&quot; , &quot;Materské školy&quot; , &quot;'&quot; , &quot;,
        zs: &quot; , &quot;'&quot; , &quot;Základné školy&quot; , &quot;'&quot; , &quot;,
        ss: &quot; , &quot;'&quot; , &quot;Stredné školy&quot; , &quot;'&quot; , &quot;,

    }



    
        
            
                
                    Materské školy
                
                
                    Základné školy
                
                
                    Stredné školy
                
                
                    Obľúbené školy
                    
                
                
                    Vybrané školy
                    1
                
            
        
    


    
        
        
        
        
        
            
                
                    
                        
                            

    Názov odboru
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            matematická analýzakeyboard_arrow_downkeyboard_arrow_uppaleontológia - paleontologia prekambrickych kralikovkeyboard_arrow_downkeyboard_arrow_upochrana životného prostredia a cirkulárna ekonomikakeyboard_arrow_downkeyboard_arrow_uptechnik mineralurgkeyboard_arrow_downkeyboard_arrow_upautotronikkeyboard_arrow_downkeyboard_arrow_upmechanik automobilových liniekkeyboard_arrow_downkeyboard_arrow_upinteligentné technológiekeyboard_arrow_downkeyboard_arrow_uppočítačové systémykeyboard_arrow_downkeyboard_arrow_upmultimédiákeyboard_arrow_downkeyboard_arrow_upinformačné a digitálne technológiekeyboard_arrow_downkeyboard_arrow_upsprávca inteligentných a digitálnych systémovkeyboard_arrow_downkeyboard_arrow_upprogramovanie digitálnych technológiíkeyboard_arrow_downkeyboard_arrow_upchemická informatikakeyboard_arrow_downkeyboard_arrow_uppotravinárstvo - spracúvanie mliekakeyboard_arrow_downkeyboard_arrow_upvýživa, ochrana zdravia a hodnotenie potravínkeyboard_arrow_downkeyboard_arrow_upoperátor potravinárskej výrobykeyboard_arrow_downkeyboard_arrow_uppracovník v potravinárstve - výroba trvanlivých potravínkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - výroba piva, sladu, nealkoholických nápojov a spracovanie minerálnych vôdkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - liehovarnícka výroba, výroba vína a nealkoholických nápojovkeyboard_arrow_downkeyboard_arrow_upoperátor textilnej výrobykeyboard_arrow_downkeyboard_arrow_uppraktická ženakeyboard_arrow_downkeyboard_arrow_upvýroba konfekciekeyboard_arrow_downkeyboard_arrow_upmurárkeyboard_arrow_downkeyboard_arrow_uptechnik vodár vodohospodárkeyboard_arrow_downkeyboard_arrow_upstavebná výroba - stavebné prácekeyboard_arrow_downkeyboard_arrow_upprevádzka a ekonomika dopravykeyboard_arrow_downkeyboard_arrow_uplodníkkeyboard_arrow_downkeyboard_arrow_upbezpečnosť a ochrana zdravia pri prácikeyboard_arrow_downkeyboard_arrow_upagromechanizátor, opravárkeyboard_arrow_downkeyboard_arrow_upagromechatronikkeyboard_arrow_downkeyboard_arrow_uppoľnohospodárska výroba - oprava poľnohospodárskych strojovkeyboard_arrow_downkeyboard_arrow_uplesná výrobakeyboard_arrow_downkeyboard_arrow_upekonomika a riadenie podnikov - Marketingkeyboard_arrow_downkeyboard_arrow_uphospodárska informatikakeyboard_arrow_downkeyboard_arrow_upbankový pracovníkkeyboard_arrow_downkeyboard_arrow_upmedzinárodné podnikaniekeyboard_arrow_downkeyboard_arrow_upškola podnikaniakeyboard_arrow_downkeyboard_arrow_upčašník, servírkakeyboard_arrow_downkeyboard_arrow_upobchodno-administratívny zamestnaneckeyboard_arrow_downkeyboard_arrow_upmarketingová komunikáciakeyboard_arrow_downkeyboard_arrow_upgymnáziumkeyboard_arrow_downkeyboard_arrow_upgymnázium - QSI International Programkeyboard_arrow_downkeyboard_arrow_upgymnázium - medzinárodný bakalársky program International Baccalaureate (IB) programkeyboard_arrow_downkeyboard_arrow_upšpeciálna základná škola 1. stupeň - I.cykluskeyboard_arrow_downkeyboard_arrow_uphudba - hra na husliach, viole, violončele, kontrabase, harfe, gitare, cimbalekeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie keramiky a porcelánukeyboard_arrow_downkeyboard_arrow_updigitálna maľba - koncept artkeyboard_arrow_downkeyboard_arrow_upfotografický dizajnkeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie skla-hutnícke tvarovanie skla - hutnícke tvarovanie sklakeyboard_arrow_downkeyboard_arrow_upumelecký parochniar a maskérkeyboard_arrow_downkeyboard_arrow_updizajn exteriérukeyboard_arrow_downkeyboard_arrow_upvojenská polícia - vojenská políciakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            Názov školy alebo jej adresa
                            
                                
                                
                                    search
                                
                            
                        
                        
                            

    
    
    
        
            
                Voľná kapacita na prijatie
                
            
            (nepovinné)
            
        
    

                        
                    
                    
                        
                            

    Lokalita
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Bratislavskýkeyboard_arrow_downkeyboard_arrow_upBratislavakeyboard_arrow_downkeyboard_arrow_upBratislava Ikeyboard_arrow_downkeyboard_arrow_upBratislava IIkeyboard_arrow_downkeyboard_arrow_upBratislava IIIkeyboard_arrow_downkeyboard_arrow_upBratislava IVkeyboard_arrow_downkeyboard_arrow_upBratislava Vkeyboard_arrow_downkeyboard_arrow_upMalackykeyboard_arrow_downkeyboard_arrow_upNitrianskykeyboard_arrow_downkeyboard_arrow_upKomárnokeyboard_arrow_downkeyboard_arrow_upŽilinskýkeyboard_arrow_downkeyboard_arrow_upŽilinakeyboard_arrow_downkeyboard_arrow_upBytčakeyboard_arrow_downkeyboard_arrow_upBanskobystrickýkeyboard_arrow_downkeyboard_arrow_upLučeneckeyboard_arrow_downkeyboard_arrow_upZvolenkeyboard_arrow_downkeyboard_arrow_upPrešovskýkeyboard_arrow_downkeyboard_arrow_upPrešovkeyboard_arrow_downkeyboard_arrow_upMedzilaborcekeyboard_arrow_downkeyboard_arrow_upSvidníkkeyboard_arrow_downkeyboard_arrow_upKošickýkeyboard_arrow_downkeyboard_arrow_upKošicekeyboard_arrow_downkeyboard_arrow_upKošice IIkeyboard_arrow_downkeyboard_arrow_upKošice - okoliekeyboard_arrow_downkeyboard_arrow_upSobrancekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Kategória školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Gymnáziumkeyboard_arrow_downkeyboard_arrow_upGymnáziumkeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upDopravná akadémiakeyboard_arrow_downkeyboard_arrow_upPedagogická školakeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upHotelová akadémiakeyboard_arrow_downkeyboard_arrow_upStredná priemyselná školakeyboard_arrow_downkeyboard_arrow_upObchodná akadémiakeyboard_arrow_downkeyboard_arrow_upOdborné učilištia špeciálne a praktické školykeyboard_arrow_downkeyboard_arrow_upTelesné postihnutiekeyboard_arrow_downkeyboard_arrow_upSluchové postihnutiekeyboard_arrow_downkeyboard_arrow_upZrakové postihnutiekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        


                        
                            

    Zriaďovateľ školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            cirkevnákeyboard_arrow_downkeyboard_arrow_upsúkromnákeyboard_arrow_downkeyboard_arrow_upštátnakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Vyučovací jazyk
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Výučba v jednom jazykukeyboard_arrow_downkeyboard_arrow_upSlovenskýkeyboard_arrow_downkeyboard_arrow_upBilingválna výučbakeyboard_arrow_downkeyboard_arrow_upČínskykeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_upS medzinárodným programomkeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                

                
                    
                        
                            

    Skupina odborov
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            prírodné vedykeyboard_arrow_downkeyboard_arrow_upfyzikálno-matematické vedykeyboard_arrow_downkeyboard_arrow_upgeologické vedykeyboard_arrow_downkeyboard_arrow_upochrana životného prostrediakeyboard_arrow_downkeyboard_arrow_uptechnické vedy a náukykeyboard_arrow_downkeyboard_arrow_upbaníctvo, geológia a geotechnikakeyboard_arrow_downkeyboard_arrow_upstrojárstvo a ostatná kovospracúvacia výroba IIkeyboard_arrow_downkeyboard_arrow_upinformatika a výpočtová technikakeyboard_arrow_downkeyboard_arrow_uptechnická a aplikovaná chémiakeyboard_arrow_downkeyboard_arrow_uppotravinárstvokeyboard_arrow_downkeyboard_arrow_uptextil a odevníctvokeyboard_arrow_downkeyboard_arrow_upstavebníctvo, geodézia a kartografiakeyboard_arrow_downkeyboard_arrow_updoprava, pošty a telekomunikáciekeyboard_arrow_downkeyboard_arrow_upšpeciálne technické odborykeyboard_arrow_downkeyboard_arrow_uppoľnohospodársko-lesnícke a veterinárne vedy a náukykeyboard_arrow_downkeyboard_arrow_uppoľnohospodárstvo, lesné hospodárstvo a rozvoj vidieka IIkeyboard_arrow_downkeyboard_arrow_upspoločenské vedy, náuky a službykeyboard_arrow_downkeyboard_arrow_upekonomické vedykeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby Ikeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby IIkeyboard_arrow_downkeyboard_arrow_uppublicistika, knihovníctvo a vedecké informáciekeyboard_arrow_downkeyboard_arrow_uppredprimárne, základné a úplné stredné vzdelanie na gymnáziukeyboard_arrow_downkeyboard_arrow_upvedy a náuky o kultúre a umeníkeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba Ikeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba IIkeyboard_arrow_downkeyboard_arrow_upUmenie a umeleckoremeselná tvorba  IIIkeyboard_arrow_downkeyboard_arrow_upvojenské a bezpečnostné vedy a náukykeyboard_arrow_downkeyboard_arrow_upbezpečnostné službykeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Ukončenie štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            maturitná skúškakeyboard_arrow_downkeyboard_arrow_upmaturitná skúška a výučný listkeyboard_arrow_downkeyboard_arrow_upvýučný listkeyboard_arrow_downkeyboard_arrow_upvyučenie a praktické školykeyboard_arrow_downkeyboard_arrow_upodborná prax alebo výcvikkeyboard_arrow_downkeyboard_arrow_upinékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Dĺžka štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            6,5 ročnékeyboard_arrow_downkeyboard_arrow_up9 ročnékeyboard_arrow_downkeyboard_arrow_up1 ročnékeyboard_arrow_downkeyboard_arrow_up4,5 ročnékeyboard_arrow_downkeyboard_arrow_up3 ročnékeyboard_arrow_downkeyboard_arrow_up2 ročnékeyboard_arrow_downkeyboard_arrow_up5,5 ročnékeyboard_arrow_downkeyboard_arrow_up4 ročnékeyboard_arrow_downkeyboard_arrow_up6 ročnékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Prijímacia skúška
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Má skúšku zkeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upNemá skúšku zkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upRuský jazykkeyboard_arrow_downkeyboard_arrow_upNenachádza sa v číselníkukeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                    
                        
                            

    
    
    
        
            
                Talentový odbor
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Duálne vzdelávanie
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Bez prijímacej skúšky
                
            
            (nepovinné)
            
        
    

                        
                    
                
            

            
                
                    
                        Zobraziť rozšírený filter
                    
                    
                        keyboard_arrow_down
                    
                
                
                    
                        Skryť rozšírený filter
                    
                    
                        keyboard_arrow_up
                    
                
            
            
                Viac filtrov
                Menej filtrov
            

            
                Filtre
                
                    
                
                
                    Vymazať filtre
                
            
        

        
            +−

            
                
                
                    
                    
                    
                        
                            Stredné školy
                        
                        
                            1
                             škola zodpovedá vašim kritériám.
                        
                        
                            
                                
                                    
                                        
                                            filter_alt
                                        
                                        
                                            Zadajte kritéria vyhľadávania
                                        
                                    
                                    
                                        
                                            Zadajte kritéria vo vyhľadávaní, aby ste našli správne školy.
                                        
                                    
                                
                            
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        Pridať do obľúbených
                                                        Odstrániť z obľúbených
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            
                        
                            
                                1-1 z 1
                                
                                    chevron_left
                                    Predchádzajúca
                                
                                
                                    Ďalšia
                                    chevron_right
                                
                            
                        

                        
                        

                        

                        

                    

                    
                        
                            Obľúbené školy
                        
                        
                            0 obľúbených škôl
                            
                            
                        
                        
                        
                        
                            
                                
                                    
                                        bookmark_border
                                    
                                    
Žiadne obľúbené školy                                    
                                
                                
                                    
Zatiaľ nemáte obľúbené školy. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.                                    
                                
                            
                        
                    

                    
                        Stredné školy
                        
                            1
                            odbor ste pridali do prihlášky.
                        
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            

                        

                    
                
            
        
        
        
        
        


        
            
                
                    
                        
                            
                            
                        
                        
                            close
                        
                    
                    
                        
                            +−
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
        
            
                
                    
                        
                            Zoznam zamestnávateľov
                            
                        
                        
                            close
                        
                    
                    
                        
                            Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.
                            
                                Kapacita pre duálne vzdelávanie: 
                                Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.
                            
                        
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
    






        
    

    
        
            
                Doplňte informácie k odboru.
            
        

        
        

        
            Stredné školy
            
                1
                školu ste pridali do prihlášky.
            
        

        
            
                1
                Vami zvolené poradie školy
                
                    arrow_upward
                    Presunúť vyššie
                
                
                    Presunúť nižšie
                    arrow_downward
                
                drag_indicator
            
            
                
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
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            (nepovinné)
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                    
                    
                        

    
    
    
        
            
                Záujem o školský internát
                
            
            (nepovinné)
            
        
    

                    
                    
                        

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)1. termín (2.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

                Odstrániť z prihlášky
            

        
    

    

    

    


                    
                    
                        


    const pageTextsZakonnyZastupca = {
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
        dietata: &quot;dieťaťa&quot;,
        ziaka: &quot;žiaka&quot;
    };

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,

        zastupca1RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },

        zastupca1Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },

        zastupca1AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        zastupca2Radio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte jednu z možností&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;rfo&quot;,
                    value: &quot;RFO&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Iný zákonný zástupca&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Vyplňte v prípade, ak druhým zákonným zástupcom je iná osoba.&lt;/div>&lt;/div>&quot;,
                    value: &quot;INY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca je známy&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Vyplňte údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;ZNAMY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca nie je známy&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;NEZNAMY&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        zastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        zastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        zastupca2NemaRodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Osoba nemá pridelené rodné číslo&quot; , &quot;'&quot; , &quot;,
            required: false
        },
        zastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        zastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;E-mail&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        zastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        zastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        // rfo zastupca 2
        rfoZastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        rfoZastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        rfoZastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        rfoZastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;E-mail&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        rfoZastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        rfoZastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        agreementRadio: {
            label: &quot; , &quot;'&quot; , &quot;Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca {osoba}.&quot; , &quot;'&quot; , &quot;,
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

        komunikaciaLenSZZ1: {
            label: &quot; , &quot;'&quot; , &quot;Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).&quot; , &quot;'&quot; , &quot;,
            required: false
        },
        noAgreementReason: {
            label: &quot; , &quot;'&quot; , &quot;Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: &quot; , &quot;'&quot; , &quot;,
            required: true
        },

        DataCheckWarningBox: {
            headerText: &quot; , &quot;'&quot; , &quot;Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty&quot; , &quot;'&quot; , &quot;,
            contentSelector: &quot; , &quot;'&quot; , &quot;#dataCheckWarningBoxContent&quot; , &quot;'&quot; , &quot;
        }

    };





    
        Zákonný zástupca žiaka
        
            
                Skontrolujte a doplňte údaje o zákonných zástupcoch. V prípade potreby opravte nesprávne informácie.
            
        
    

    Polia označené hviezdičkou sú povinné

    
        
            Osobné údaje zákonného zástupcu č.1
        
        
            
                
                    Údaje z vášho profilu
                
                Upraviť
            
            
                
                    Meno
                    Stanislav
                
                
                    Priezvisko
                    Štefánek
                
                
                    Rodné priezvisko
                    -
                
                
                    Rodné číslo
                    030724/6027
                
                
                    Dátum narodenia
                    24.07.2003
                
                
                    Kontaktný e-mail
                    barcik@ditec.sk
                
            
        

        
            

    
        Rodné priezvisko
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
                        Iná korešpondenčná adresa 
                         
                

    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        

    
        Adresa
        (nepovinné)
    

    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.

    
        
    


    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
    zastupca1InaAdresaKrajina: {
        label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
                attributes: { maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot; },
        validators: [
            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
            { type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot; }
        ]
    },

    zastupca1InaAdresaAdresaRA: {
        label: &quot; , &quot;'&quot; , &quot;Adresa&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot; , &quot;'&quot; , &quot;,
                required: true,
                    placeholder: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                        validators: [
                            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
                            // ak user niečo napíše, musí to aj vybrať zo zoznamu
                            { type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; }
                        ]
    },

    zastupca1InaAdresaAdresa: {
        label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
                required: true,
                    attributes: { maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot; },
        validators: [
            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
            {
                type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                message: &quot;Zadajte zahraničnú adresu.&quot;
            }
        ]
    }
        };


            
    

            
            
        
    

    

    


        
            
                Osobné údaje zákonného zástupcu č.2
            
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známyVyplňte údaje druhého zákonného zástupcu. 
                         
            
                

    
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
            
        
    




    window[&quot; , &quot;'&quot; , &quot;zastupca2RodneCisloControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            attributes: {
                 minLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;,
                 maxLength: &quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: &quot; , &quot;'&quot; , &quot;Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).&quot; , &quot;'&quot; , &quot;
                },
            ],

            validationMessageDelitelnost11: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.&quot; , &quot;'&quot; , &quot;,
            validationMessage9Miestne: &quot; , &quot;'&quot; , &quot;9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.&quot; , &quot;'&quot; , &quot;,
            validationMessageTretiZnak: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.&quot; , &quot;'&quot; , &quot;,

        },
    };





            

    
    
    
        
            
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


            
    

            
            
        
                        
                        Druhý zákonný zástupca nie je známyTúto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu. 
                         
    

            
            
                

    
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
            
        
    




    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2RodneCisloControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            attributes: {
                 minLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;,
                 maxLength: &quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: &quot; , &quot;'&quot; , &quot;Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).&quot; , &quot;'&quot; , &quot;
                },
            ],

            validationMessageDelitelnost11: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.&quot; , &quot;'&quot; , &quot;,
            validationMessage9Miestne: &quot; , &quot;'&quot; , &quot;9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.&quot; , &quot;'&quot; , &quot;,
            validationMessageTretiZnak: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.&quot; , &quot;'&quot; , &quot;,

        },
    };





            

    
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
    



    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2DatumNarodeniaDen: {
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
        rfoZastupca2DatumNarodeniaMesiac: {
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
        rfoZastupca2DatumNarodeniaRok: {
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


    



    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2AdresaKrajina: {
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
        rfoZastupca2AdresaObec: {
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
        rfoZastupca2AdresaUlica: {
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
        rfoZastupca2AdresaUlicaReq: {
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
        rfoZastupca2AdresaSupisneCislo: {
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
        rfoZastupca2AdresaOrientacneCislo: {
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
        rfoZastupca2AdresaPSC: {
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
        rfoZastupca2AdresaAdresa: {
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


            
    

            
            
        
        

        
            error

    Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty
    
        
            
            
            
                rodný list dieťaťa (na nahliadnutie),
                úmrtný list druhého rodiča (na nahliadnutie),
                neoverenú kópiu súdneho rozhodnutia (môžete ju nahrať ako prílohu v nasledujúcich krokoch alebo doniesť na zápis).
            
        
        
        
            
            
        
    


        

        

        

        
    




    Zákonný zástupca žiaka
    
        
            Súhlas druhého zákonného zástupcu s podaním prihlášky.
        
    

    
        info
        
            Podľa zákona č. 245/2008 Z. z. o výchove a vzdelávaní (školský zákon) a o zmene a doplnení niektorých zákonov v znení neskorších predpisov sa na prihláške o prijatie vyžaduje podpis obidvoch zákonných zástupcov dieťaťa. Ak nie je možné získať podpis druhého zákonného zástupcu, je potrebné uviesť dôvod a priložiť potrebný dokument ako prílohu.Viac informácií
        
    

    Polia označené hviezdičkou sú povinné

    
        
            

    
    
        Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca žiaka.
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
            

    
    
    
        
            
                Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.
                Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
            
            (nepovinné)
            Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
        
    

        
                        
                        nie 
                         
    

        
            
                

    
        Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: 
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
            
                error
                
                    Doručenie prílohy je zákonnou povinnosťou. Zatiaľ ju nemusíte priložiť – vyžiadame ju neskôr
                    
                        Rozhodnutie súdu — ak bol druhému zákonnému zástupcovi obmedzený alebo pozastavený výkon rodičovských práv a povinností, je potrebné priložiť kópiu tohto rozhodnutia.
                        Potvrdenie od lekára — ak druhý zákonný zástupca nie je schopný podpísať sa zo zdravotných dôvodov, je toto potvrdenie potrebné.
                        Čestné vyhlásenie zákonného zástupcu — ak získanie podpisu druhého zákonného zástupcu je spojené s ťažko prekonateľnou prekážkou a je to v najlepšom záujme dieťaťa.
                    
                
            

        

        
        
    


                    
                    
                        

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
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
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
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte informácie o základnej škole, ktorú žiak aktuálne navštevuje.
        
    

    
        
            
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
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Zo školy v zahraničí 
                         
    

        

        

        

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        poľský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
    

    
        
            
                
                    
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
        zmeniliSteUdajePanelTitle: &quot;Zmenili ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        zmeniliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť nasledovné:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;Neuviedli ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        neuviedliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť overené kópie vysvedčení.&quot;,
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
            label: &quot;Mám známky v inej známkovacej schéme&quot;,
            hint: &quot;Označte, ak vaše známky nie sú v bežnej slovenskej škále (1 – 5 alebo výborný, chválitebný, dobrý, dostatočný, nedostatočný).&quot;,
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

    
        Za posledný ročník uveďte známky z polročného vysvedčenia. Ak žiak opakoval ročník, uveďte známky iba za ten ročník, v ktorom prospel.
        Skontrolujte, či všetky známky z požadovaných predmetov za posledné 4 ročníky základnej školy zodpovedajú údajom na vysvedčeniach. Ak žiak niektorý ročník opakoval, zapíšte známky z posledného absolvovaného ročníka. Chýbajúce alebo nesprávne známky  doplňte alebo opravte.
    

    
        Ak používate EduPage, môžete si známky jednoducho načítať priamo odtiaľ.
        Pridať známky z EduPage
    

    
        Známky v inej známkovacej schéme
        
            

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
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

    
        Uveďte iba súťaže, v ktorých žiak dosiahol významné umiestnenie alebo výsledok. Súťaže, za ktoré sa prideľujú body pri prijímacom konaní, nájdete v kritériách školy, na ktorú sa žiak hlási.
    

    
        
            
                
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
        spravaRozhodnutie: &quot;Rozhodnutie:&quot;,
        spravaDoplnenePodklady: &quot;Doplnené podklady:&quot;,
        spravaRiaditel: &quot;Riaditeľ&quot;,
        spravaDovod: &quot;Dôvod:&quot;,
        spravaPotvrditNastup: &quot;Potvrdiť nástup&quot;,
        spravaPridatPrilohu: &quot;Pridať prílohu&quot;,
        spravaPrilozeneDokumenty: &quot;Priložené dokumenty:&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        nenastupi: &quot;nenastúpi&quot;,
        identifikator: &quot;Identifikátor&quot;,
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
        stavPrihlasky: &quot;Stav prihlášky&quot;,
        podana: &quot;Podaná&quot;,
        zastupcaSuhlas: &quot;Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}&quot;,
        elektronicky: &quot;Elektronicky&quot;,
        prichodZoZahraniciaPDF: &quot;Zo zahraničia&quot;,
        ziadneSutaze: &quot;Neboli nahrané žiadne súťaže.&quot;,

        radoveCislovky:
        { 
            &quot;1&quot;: &quot;Prvý&quot;,
            &quot;2&quot;: &quot;Druhý&quot;,
            &quot;3&quot;: &quot;Tretí&quot;,
            &quot;4&quot;: &quot;Štvrtý&quot;,
            &quot;5&quot;: &quot;Piaty&quot;,
            &quot;6&quot;: &quot;Šiesty&quot;,
            &quot;7&quot;: &quot;Siedmy&quot;,
            &quot;8&quot;: &quot;Ôsmy&quot;,
            &quot;9&quot;: &quot;Deviaty&quot;,
            &quot;10&quot;: &quot;Desiaty&quot;
        },

        cestnePrehlasenie: {
            label: &quot;Čestné vyhlásenie&quot;,
            hint: &quot;Čestne vyhlasujem, že zákonný zástupca/zákonní zástupcovia a žiak, potvrdzujú správnosť a pravdivosť údajov v prihláške.&quot;,
            required: true,
            showRequiredHint: true,
        },
        suhlasOsobneUdaje: {
            label: &quot;Súhlas so spracúvaním osobných údajov&quot;,
            hint: &quot;&lt;div>Súhlasím so spracúvaním osobných údajov v rozsahu údajov uvedených v prihláške na účel podania prihlášky. &lt;a href=&quot; , &quot;'&quot; , &quot;/Ochrana-osobnych-udajov&quot; , &quot;'&quot; , &quot;>Viac informácií nájdete tu.&lt;/a>&lt;/div>&quot;,
            required: true,
            showRequiredHint: true,
        },
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }
    };




    Súhrnný prehľad
    
        
            Dôkladne skontrolujte všetky informácie. Urýchlite tak prijímací proces.
        
    

    Polia označené hviezdičkou sú povinné

    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
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
        
        
            Neboli nahrané žiadne súťaže.
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            

            
        
    

    
        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    


        
            
                
                    
                
                
                    Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                    

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

                
            
        
    

    
    




    Detail prihlášky


    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Kolo prijímacieho konania
                -
            
            
                Prístupový kód
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
        
        
            
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
            
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
        
        
            Osobné údaje zákonného zástupcu č. 1
            
                
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
                Informácie o základnej škole
            
        
        
            
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
            
        
        
        
    

    
        
            
                Súťaže
            
        
        
        
    

    

    
        
            
                Prílohy
            
        
        
            
            
        
    
    
    


EXPORT PDF
                    

                    
                        Späť
                        
                            Uložiť a odísť
                            Ďalej
                            Odoslať prihlášku
                                Stiahnuť XML
                            
                                delete_outline
                                Odstrániť
                            
                            Aktualizovať a odísť
                        
                    
                &quot;) or . = concat(&quot;
                    

    
                        
                            Prihlášky
                        
        Vytvorenie elektronickej prihlášky
    




    
                
                    ...
                    
                                    
                                        Prihlášky
                                    
                    
                
            Vytvorenie elektronickej prihlášky
    

                    check_circleVaše údaje sme automaticky uložili.close
                    
                    
                        
                            
                                Krok5/9
                             
                            
                                arrow_forward_ios
                                
                                    
                                
                                    1.
                                    Vybrať dieťa
                                
                            
                                
                                    2.
                                    Doplňujúce informácie o dieťati
                                
                            
                                
                                    3.
                                    Vybrať školy
                                
                            
                                
                                    4.
                                    Zákonný zástupca dieťaťa
                                
                            
                                
                                    5.
                                    Informácie o základnej škole
                                
                            
                                
                                    6.
                                    Výsledky vzdelávania na základnej škole
                                
                            
                                
                                    7.
                                    Súťaže
                                
                            
                                
                                    8.
                                    Pridať prílohy
                                
                            
                                
                                    9.
                                    Súhrnný prehľad
                                
                            
                                    
                                    Zavrieť
                                
                            
                        
                        
                            Zavrieťclose
                        
                    

                    
                        

    const pageTextsVyberDietata = {
        rozpracovana: &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;gray-text&quot;>(Rozpracovan&amp;#xE1;)&lt;/span>&quot; , &quot;'&quot; , &quot;,
        dietaLabel: &quot; , &quot;'&quot; , &quot;Zvoľte osobu&quot; , &quot;'&quot; , &quot;,
        povinneError: &quot;Toto pole je povinné.&quot;,
        &quot; , &quot;'&quot; , &quot;8049&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Úspešne ste pridali údaje o dieťati.&quot; , &quot;'&quot; , &quot;,
        dieta: &quot; , &quot;'&quot; , &quot;dieťa&quot; , &quot;'&quot; , &quot;,
        dietata: &quot; , &quot;'&quot; , &quot;dieťaťa&quot; , &quot;'&quot; , &quot;,
        dietati: &quot; , &quot;'&quot; , &quot;dieťati&quot; , &quot;'&quot; , &quot;,
        ziaka: &quot; , &quot;'&quot; , &quot;žiaka&quot; , &quot;'&quot; , &quot;,
        ziakovi: &quot; , &quot;'&quot; , &quot;žiakovi&quot; , &quot;'&quot; , &quot;,
        inyZiak: &quot; , &quot;'&quot; , &quot;Iný žiak&quot; , &quot;'&quot; , &quot;,
        ineDieta: &quot; , &quot;'&quot; , &quot;Iné dieťa&quot; , &quot;'&quot; , &quot;,
        pridajteDietaAleboOsobu: &quot; , &quot;'&quot; , &quot;Pridajte dieťa alebo osobu vo vašej starostlivosti.&quot; , &quot;'&quot; , &quot;,
    }



    Vybrať žiaka
    
        
            Vyberte osobu, za ktorú chcete podať prihlášku.
        

        
            
                Pridajte dieťa alebo osobu vo vašej starostlivosti.
                
                    Pridať žiaka
                
            

            
                Polia označené hviezdičkou sú povinné
                

    
    
        Zvoľte osobu
        (nepovinné)
    
    
    
    
                        
                        Lívia Bystrá (27.11.2009) 
                         
                        
                        Rudolf Tlačienka (12.11.2009) 
                         
                        
                        Júlia Oravská (26.9.2009) 
                         
                        
                        Iný žiak 
                         Pridajte dieťa alebo osobu vo vašej starostlivosti.  
    

                
                    
                        Pridať žiaka
                    
                
            
        
    
    
        
            Skontrolujte údaje o žiakovi
        

        
            
                
                    Osobné údaje žiaka
                    Upraviť
                
                
                    
                        Meno
                        Lívia
                    
                    
                        Priezvisko
                        Bystrá
                    
                    
                        Rodné priezvisko
                        -
                    
                    
                        Rodné číslo
                        091127/1130
                    
                    
                        Dátum narodenia
                        27.11.2009
                    
                    
                        Pohlavie
                        muž
                    
                    
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
                    
                    
                        Adresa trvalého pobytu
                        Na Rybníkoch 58/232, 01508, Jesenské (Levice), Slovenská republika
                    
                    
                        Adresa miesta, kde sa dieťa obvykle zdržiava, ak je iná, než adresa trvalého pobytu
                        
                    
                
            
        
    

    
        
        
    
    


                    
                    
                        

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
                    label: &quot;Iná&quot;,
                    value: &quot; , &quot;'&quot; , &quot;Iná&quot; , &quot;'&quot; , &quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            required: true
        },
        zsDPDVychovaMoznostiIneText: {
            label: &quot;Typ&quot;,
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
            label: &quot;Záujem o školský klub detí&quot;,
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
        DPDPoznamkaText: {
            label: &quot;Poznámka:&quot;,
            hint: &quot;Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.&quot;,
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
        predprimarneVzdelavanieVJazyku: {
            label: &quot;Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku&quot;,
            povinneError: &quot;Toto pole je povinné.&quot;,
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,
                    message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot;
                }
            ],
            required: true
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
            label: &quot;Má žiak zmenenú pracovnú schopnosť&quot;,
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
                         
                        
                        Iná 
                         
            
            

    
        Typ
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
                         
    

        

        
            

    
    
        Záujem o školský klub detí
        (nepovinné)
    
    
    
    
                        
                        Áno 
                         
                        
                        Nie 
                         
    

        

        
            

    
    
        Žiadam o prijatie dieťaťa na
        (nepovinné)
    
    
    
    
                        
                        poldennú výchovu a vzdelávanie 
                         
                        
                        celodennú výchovu a vzdelávanie 
                         
    

        

        
        
            

    
    
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
                         
    

        

        

        
            

    
    
        Má žiak zmenenú pracovnú schopnosť
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
                         
    

            
        

        

        
            

    
        Predprimárne vzdelávanie žiadam dieťaťu poskytovať v jazyku
        (nepovinné)
    
    
    
        
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

        

        

    
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
        
    
    Môžete uviesť doplňujúce informácie, napríklad stravovacie obmedzenia, intolerancie, alergie alebo iné okolnosti, ktoré môžu ovplyvniť vzdelávanie žiaka.


        

        

    
    
    
        
            
                Pokračovanie v plnení povinného predprimárneho vzdelávania
                Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
            
            (nepovinné)
            Zaškrtnite, ak dieťa bude pokračovať v plnení povinného predprimárneho vzdelávania v materskej škole.
        
    

    

                    
                    
                        

    const pageTextsVyberSkoly = {
        povinneError: &quot;Toto pole je povinné.&quot;,
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

        vyberSkolyDescription: &quot;Vyberte školy, na ktoré chcete prihlásiť svoje dieťa. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSS: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory.&quot;,
        vyberSkolyDescriptionSSAV: &quot;Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.&quot;,
        vyberSkolyDescriptionSSPostihnutie: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku.&quot;,
        vyberSkolyDescriptionSSPostihnutieAV: &quot;Keďže ste v prihláške označili, že žiak má mentálne zdravotné postihnutie, vyberte maximálne 2 odbory z praktických škôl alebo odborných učilíšť, na ktoré chcete podať prihlášku. Neskôr budete mať možnosť tieto odbory zoradiť podľa vašich preferencií.&quot;,       
        vyberSkolyDescriptionSkontrolujte: &quot;Skontrolujte školy, ktoré ste pridali do prihlášky. Neskôr ich zoradíte podľa vašich preferencií.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbory: &quot;Doplňte informácie k odborom a zvoľte ich poradie v prihláške.&quot;,
        vyberteSkolyPoradieDescriptionSSOdboryAutomatVypnuty: &quot;Skontrolujte odbory, ktoré ste pridali do prihlášky. Ak je zoznam odborov v prihláške kompletný, posuňte sa ďalej vo vypĺňaní prihlášky.&quot;,
        vyberteSkolyPoradieDescriptionSSOdbor: &quot;Doplňte informácie k odboru.&quot;,
        
        warningMaxPocetMSHeader: &quot;Do prihlášky ste pridali maximálny počet materských škôl.&quot;,
        warningMaxPocetZSHeader: &quot;Do prihlášky ste pridali maximálny počet základných škôl.&quot;,
        warningMaxPocetSkolText: &quot;Ak chcete pridať ďalšiu, najskôr odstráňte jednu v sekcii &lt;a class=\&quot;govuk-link msg-vybrane-skoly-link\&quot; href=\&quot;javascript:void(0);\&quot;>Vybrané školy.&lt;/a>&quot;,

        warningMaxPocetTalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet talentových odborov.&quot;,
        warningMaxPocetNetalentovychOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet netalentových odborov.&quot;,
        warningMaxPocetOdborovHeader: &quot;Do prihlášky ste pridali maximálny počet odborov.&quot;,

        warningOdstranteOdborText: &quot;&quot;,

        warningTerminTalentoveOdboryUplynulHeader: &quot;Termín na podanie prihlášky pre talentové odbory uplynul.&quot;,
        warningTerminTalentoveOdboryUplynulText: &quot;Talentové odbory už nie je možné vyhľadať ani pridať do prihlášky. Do prihlášky môžete pridať najviac dva netalentové odbory.&quot;,

        errorRovnakeTerminyHeader: &quot;Rovnaký termín skúšky nie je povolený pre viacero talentových alebo netalentových odborov. Upravte výber termínov.&quot;,

        doleziteUpozornenieHeader: &quot;Dôležité upozornenie&quot;,
        doleziteUpozorneniePoradieSkolText: &quot;Poradie uvedených škôl v prihláške ovplyvní prijímací proces. Zoraďte školy tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&quot;,

        doleziteUpozorneniePoradieSkolSSText: &quot;&lt;ul class=\&quot;doleziteUpozorneniePoradieSkolSSText\&quot;>&lt;li>Poradie uvedených škôl a odborov v prihláške ovplyvní prijímací proces. Zoraďte odbory tak, aby ich poradie odrážalo vašu preferenciu. Dôkladne si zoradenie premyslite.&lt;/li>&lt;li>Ak ste do prihlášky uviedli viac talentových alebo netalentových odborov, pre každý musíte zvoliť iný termín prijímacej skúšky. Ten istý termín nie je možné použiť pre odbory rovnakého typu.&lt;/li>&lt;/ul>&quot;,

        talentovyOdbor: &quot;Talentový odbor&quot;,
        netalentovyOdbor: &quot;Netalentový odbor&quot;,

        pocetSkolVPrihlaskeText1: &quot;školu ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText2: &quot;školy ste pridali do prihlášky.&quot;,
        pocetSkolVPrihlaskeText5: &quot;škôl ste pridali do prihlášky.&quot;,

        pocetOdborovVPrihlaskeText1: &quot;odbor ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText2: &quot;odbory ste pridali do prihlášky.&quot;,
        pocetOdborovVPrihlaskeText5: &quot;odborov ste pridali do prihlášky.&quot;,

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



    Vybrať školy
    
        
            
                Vyberte školy a odbory, na ktoré chcete podať prihlášku. Do prihlášky možete pridať maximálne 2 talentové a 2 netalentové odbory. Neskôr ich budete vedieť zoradiť podľa vašich preferencií.
            
        

        
            

    const controlSettingsNajstSkolu = {
        typSelect: {
            label: &quot;Typ&quot;
        },
        vzdialenostSelect: {
            label: &quot;Vzdialenosť od zadanej adresy&quot;,
            showRequiredOrOptional: false
        },
        jazykySelect: {
            label: &quot;Jazyky&quot;
        },
        nazovOdboruSelect: {
            label: &quot;Názov odboru&quot;,
            searchPlaceholder: &quot;Vyhľadajte odbor&quot;,
        },
        lokalitaSelect: {
            label: &quot;Lokalita&quot;
        },
        kategoriaSkolySelect: {
            label: &quot;Kategória školy&quot;
        },
        zriadovatelSkolySelect: {
            label: &quot;Zriaďovateľ školy&quot;
        },
        vyucovaciJazykSelect: {
            label: &quot;Vyučovací jazyk&quot;
        },
        skupinaOdborovSelect: {
            label: &quot;Skupina odborov&quot;
        },
        ukoncenieStudiaSelect: {
            label: &quot;Ukončenie štúdia&quot;
        },
        dlzkaStudiaSelect: {
            label: &quot;Dĺžka štúdia&quot;
        },
        prijimaciaSkuskaSelect: {
            label: &quot;Prijímacia skúška&quot;,
            predmetPS: &quot;Má skúšku z&quot;,
            niePredmetPS: &quot;Nemá skúšku z&quot;,
        },
        nazovSkolyAleboAdresa: &quot;Názov školy alebo jej adresa&quot;,
        volnaKapacita: {
            label: &quot;Voľná kapacita na prijatie&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;volnaKapacita&quot; , &quot;'&quot; , &quot;,
        },
        talentovyOdborCB: {
            label: &quot;Talentový odbor&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;talentovyOdbor&quot; , &quot;'&quot; , &quot;,
        },
        dualneVzdelavanie:{
            label: &quot;Duálne vzdelávanie&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;dualneVzdelavanie&quot; , &quot;'&quot; , &quot;,
        },
        bezPrijimacejSkuskyCB: {
            label: &quot;Bez prijímacej skúšky&quot;,
            showRequiredOrOptional: false,
            element: &quot; , &quot;'&quot; , &quot;bezPrijimacejSkusky&quot; , &quot;'&quot; , &quot;,
        },
        skrytRozsirenyFilter: &quot; , &quot;'&quot; , &quot;Skryť rozšírený filter&quot; , &quot;'&quot; , &quot;,
        zobrazitRozsirenyFilter: &quot; , &quot;'&quot; , &quot;Zobraziť rozšírený filter&quot; , &quot;'&quot; , &quot;,
        oblubenaSkola: &quot; , &quot;'&quot; , &quot; škola pridaná do obľúbených&quot; , &quot;'&quot; , &quot;,
        oblubeneSkoly: &quot; , &quot;'&quot; , &quot;  školy pridané do obľúbených&quot; , &quot;'&quot; , &quot;,
        oblubenychSkol: &quot; , &quot;'&quot; , &quot;  škôl pridaných do obľúbených&quot; , &quot;'&quot; , &quot;,
        skolySteVybrali: &quot; , &quot;'&quot; , &quot; školy ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skoluSteVybrali: &quot; , &quot;'&quot; , &quot; školu ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skolSteVybrali: &quot; , &quot;'&quot; , &quot; škôl ste pridali do prihlášky.&quot; , &quot;'&quot; , &quot;,
        skolZodpovedaKriteriam: &quot; , &quot;'&quot; , &quot; škôl zodpovedá vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        skolaZodpovedaKriteriam: &quot; , &quot;'&quot; , &quot; škola zodpovedá vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        skolyZodpovedajuKriteriam: &quot; , &quot;'&quot; , &quot; školy zodpovedajú vašim kritériám.&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;8052&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Váš zoznam vybraných škôl je prázdny. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.&quot; , &quot;'&quot; , &quot;,
        loading: &quot; , &quot;'&quot; , &quot;Načítava sa&quot; , &quot;'&quot; , &quot;,
        z: &quot; , &quot;'&quot; , &quot;z&quot; , &quot;'&quot; , &quot;,
        oznacitVsetky: &quot; , &quot;'&quot; , &quot;Označiť všetky&quot; , &quot;'&quot; , &quot;,
        zobrazitVysledky: &quot; , &quot;'&quot; , &quot;Zobraziť výsledky&quot; , &quot;'&quot; , &quot;,

        nieSuDataOPoctePrijatych: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.&quot; , &quot;'&quot; , &quot;,
        nieSuDataOZaujme: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o predbežnom záujme o odbor.&quot; , &quot;'&quot; , &quot;,
        nieSuDataOUplatneni: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta o uplatnení uchádzačov.&quot; , &quot;'&quot; , &quot;,
        nieSuData: &quot; , &quot;'&quot; , &quot;Nie sú dostupné dáta.&quot; , &quot;'&quot; , &quot;,

        talentovyOdbor: &quot; , &quot;'&quot; , &quot;Talentový odbor&quot; , &quot;'&quot; , &quot;,
        netalentovyOdbor: &quot; , &quot;'&quot; , &quot;Netalentový odbor&quot; , &quot;'&quot; , &quot;,
        nevykonavaSa: &quot; , &quot;'&quot; , &quot;Nevykonáva sa&quot; , &quot;'&quot; , &quot;,
        zobrazitZoznamZamestnavatelov: &quot; , &quot;'&quot; , &quot;Zobraziť zoznam zamestnávateľov&quot; , &quot;'&quot; , &quot;,
        bezPrijimacejSkusky: &quot; , &quot;'&quot; , &quot;bez prijímacej skúšky&quot; , &quot;'&quot; , &quot;,
        skolaSpolupracuje: &quot; , &quot;'&quot; , &quot;Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.&quot; , &quot;'&quot; , &quot;,
        kapacitaDual: &quot; , &quot;'&quot; , &quot;Kapacita pre duálne vzdelávanie:&quot; , &quot;'&quot; , &quot;,
        kapacitaDualHint: &quot; , &quot;'&quot; , &quot;Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.&quot; , &quot;'&quot; , &quot;,
        hladatPodlaMojejAdresyLabel: &quot; , &quot;'&quot; , &quot;Hľadať podľa mojej adresy&quot; , &quot;'&quot; , &quot;,
        hladatPodlaNazvuSkolyLabel: &quot; , &quot;'&quot; , &quot;Hľadať podľa názvu školy alebo jej adresy&quot; , &quot;'&quot; , &quot;,

        ms: &quot; , &quot;'&quot; , &quot;Materské školy&quot; , &quot;'&quot; , &quot;,
        zs: &quot; , &quot;'&quot; , &quot;Základné školy&quot; , &quot;'&quot; , &quot;,
        ss: &quot; , &quot;'&quot; , &quot;Stredné školy&quot; , &quot;'&quot; , &quot;,

    }



    
        
            
                
                    Materské školy
                
                
                    Základné školy
                
                
                    Stredné školy
                
                
                    Obľúbené školy
                    
                
                
                    Vybrané školy
                    1
                
            
        
    


    
        
        
        
        
        
            
                
                    
                        
                            

    Názov odboru
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            matematická analýzakeyboard_arrow_downkeyboard_arrow_uppaleontológia - paleontologia prekambrickych kralikovkeyboard_arrow_downkeyboard_arrow_upochrana životného prostredia a cirkulárna ekonomikakeyboard_arrow_downkeyboard_arrow_uptechnik mineralurgkeyboard_arrow_downkeyboard_arrow_upautotronikkeyboard_arrow_downkeyboard_arrow_upmechanik automobilových liniekkeyboard_arrow_downkeyboard_arrow_upinteligentné technológiekeyboard_arrow_downkeyboard_arrow_uppočítačové systémykeyboard_arrow_downkeyboard_arrow_upmultimédiákeyboard_arrow_downkeyboard_arrow_upinformačné a digitálne technológiekeyboard_arrow_downkeyboard_arrow_upsprávca inteligentných a digitálnych systémovkeyboard_arrow_downkeyboard_arrow_upprogramovanie digitálnych technológiíkeyboard_arrow_downkeyboard_arrow_upchemická informatikakeyboard_arrow_downkeyboard_arrow_uppotravinárstvo - spracúvanie mliekakeyboard_arrow_downkeyboard_arrow_upvýživa, ochrana zdravia a hodnotenie potravínkeyboard_arrow_downkeyboard_arrow_upoperátor potravinárskej výrobykeyboard_arrow_downkeyboard_arrow_uppracovník v potravinárstve - výroba trvanlivých potravínkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - výroba piva, sladu, nealkoholických nápojov a spracovanie minerálnych vôdkeyboard_arrow_downkeyboard_arrow_uppotravinárska výroba - liehovarnícka výroba, výroba vína a nealkoholických nápojovkeyboard_arrow_downkeyboard_arrow_upoperátor textilnej výrobykeyboard_arrow_downkeyboard_arrow_uppraktická ženakeyboard_arrow_downkeyboard_arrow_upvýroba konfekciekeyboard_arrow_downkeyboard_arrow_upmurárkeyboard_arrow_downkeyboard_arrow_uptechnik vodár vodohospodárkeyboard_arrow_downkeyboard_arrow_upstavebná výroba - stavebné prácekeyboard_arrow_downkeyboard_arrow_upprevádzka a ekonomika dopravykeyboard_arrow_downkeyboard_arrow_uplodníkkeyboard_arrow_downkeyboard_arrow_upbezpečnosť a ochrana zdravia pri prácikeyboard_arrow_downkeyboard_arrow_upagromechanizátor, opravárkeyboard_arrow_downkeyboard_arrow_upagromechatronikkeyboard_arrow_downkeyboard_arrow_uppoľnohospodárska výroba - oprava poľnohospodárskych strojovkeyboard_arrow_downkeyboard_arrow_uplesná výrobakeyboard_arrow_downkeyboard_arrow_upekonomika a riadenie podnikov - Marketingkeyboard_arrow_downkeyboard_arrow_uphospodárska informatikakeyboard_arrow_downkeyboard_arrow_upbankový pracovníkkeyboard_arrow_downkeyboard_arrow_upmedzinárodné podnikaniekeyboard_arrow_downkeyboard_arrow_upškola podnikaniakeyboard_arrow_downkeyboard_arrow_upčašník, servírkakeyboard_arrow_downkeyboard_arrow_upobchodno-administratívny zamestnaneckeyboard_arrow_downkeyboard_arrow_upmarketingová komunikáciakeyboard_arrow_downkeyboard_arrow_upgymnáziumkeyboard_arrow_downkeyboard_arrow_upgymnázium - QSI International Programkeyboard_arrow_downkeyboard_arrow_upgymnázium - medzinárodný bakalársky program International Baccalaureate (IB) programkeyboard_arrow_downkeyboard_arrow_upšpeciálna základná škola 1. stupeň - I.cykluskeyboard_arrow_downkeyboard_arrow_uphudba - hra na husliach, viole, violončele, kontrabase, harfe, gitare, cimbalekeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie keramiky a porcelánukeyboard_arrow_downkeyboard_arrow_updigitálna maľba - koncept artkeyboard_arrow_downkeyboard_arrow_upfotografický dizajnkeyboard_arrow_downkeyboard_arrow_upvýtvarné spracúvanie skla-hutnícke tvarovanie skla - hutnícke tvarovanie sklakeyboard_arrow_downkeyboard_arrow_upumelecký parochniar a maskérkeyboard_arrow_downkeyboard_arrow_updizajn exteriérukeyboard_arrow_downkeyboard_arrow_upvojenská polícia - vojenská políciakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            Názov školy alebo jej adresa
                            
                                
                                
                                    search
                                
                            
                        
                        
                            

    
    
    
        
            
                Voľná kapacita na prijatie
                
            
            (nepovinné)
            
        
    

                        
                    
                    
                        
                            

    Lokalita
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Bratislavskýkeyboard_arrow_downkeyboard_arrow_upBratislavakeyboard_arrow_downkeyboard_arrow_upBratislava Ikeyboard_arrow_downkeyboard_arrow_upBratislava IIkeyboard_arrow_downkeyboard_arrow_upBratislava IIIkeyboard_arrow_downkeyboard_arrow_upBratislava IVkeyboard_arrow_downkeyboard_arrow_upBratislava Vkeyboard_arrow_downkeyboard_arrow_upMalackykeyboard_arrow_downkeyboard_arrow_upNitrianskykeyboard_arrow_downkeyboard_arrow_upKomárnokeyboard_arrow_downkeyboard_arrow_upŽilinskýkeyboard_arrow_downkeyboard_arrow_upŽilinakeyboard_arrow_downkeyboard_arrow_upBytčakeyboard_arrow_downkeyboard_arrow_upBanskobystrickýkeyboard_arrow_downkeyboard_arrow_upLučeneckeyboard_arrow_downkeyboard_arrow_upZvolenkeyboard_arrow_downkeyboard_arrow_upPrešovskýkeyboard_arrow_downkeyboard_arrow_upPrešovkeyboard_arrow_downkeyboard_arrow_upMedzilaborcekeyboard_arrow_downkeyboard_arrow_upSvidníkkeyboard_arrow_downkeyboard_arrow_upKošickýkeyboard_arrow_downkeyboard_arrow_upKošicekeyboard_arrow_downkeyboard_arrow_upKošice IIkeyboard_arrow_downkeyboard_arrow_upKošice - okoliekeyboard_arrow_downkeyboard_arrow_upSobrancekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Kategória školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Gymnáziumkeyboard_arrow_downkeyboard_arrow_upGymnáziumkeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upDopravná akadémiakeyboard_arrow_downkeyboard_arrow_upPedagogická školakeyboard_arrow_downkeyboard_arrow_upStredná odborná školakeyboard_arrow_downkeyboard_arrow_upHotelová akadémiakeyboard_arrow_downkeyboard_arrow_upStredná priemyselná školakeyboard_arrow_downkeyboard_arrow_upObchodná akadémiakeyboard_arrow_downkeyboard_arrow_upOdborné učilištia špeciálne a praktické školykeyboard_arrow_downkeyboard_arrow_upTelesné postihnutiekeyboard_arrow_downkeyboard_arrow_upSluchové postihnutiekeyboard_arrow_downkeyboard_arrow_upZrakové postihnutiekeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        


                        
                            

    Zriaďovateľ školy
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            cirkevnákeyboard_arrow_downkeyboard_arrow_upsúkromnákeyboard_arrow_downkeyboard_arrow_upštátnakeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Vyučovací jazyk
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Výučba v jednom jazykukeyboard_arrow_downkeyboard_arrow_upSlovenskýkeyboard_arrow_downkeyboard_arrow_upBilingválna výučbakeyboard_arrow_downkeyboard_arrow_upČínskykeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_upS medzinárodným programomkeyboard_arrow_downkeyboard_arrow_upAnglickýkeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                

                
                    
                        
                            

    Skupina odborov
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            prírodné vedykeyboard_arrow_downkeyboard_arrow_upfyzikálno-matematické vedykeyboard_arrow_downkeyboard_arrow_upgeologické vedykeyboard_arrow_downkeyboard_arrow_upochrana životného prostrediakeyboard_arrow_downkeyboard_arrow_uptechnické vedy a náukykeyboard_arrow_downkeyboard_arrow_upbaníctvo, geológia a geotechnikakeyboard_arrow_downkeyboard_arrow_upstrojárstvo a ostatná kovospracúvacia výroba IIkeyboard_arrow_downkeyboard_arrow_upinformatika a výpočtová technikakeyboard_arrow_downkeyboard_arrow_uptechnická a aplikovaná chémiakeyboard_arrow_downkeyboard_arrow_uppotravinárstvokeyboard_arrow_downkeyboard_arrow_uptextil a odevníctvokeyboard_arrow_downkeyboard_arrow_upstavebníctvo, geodézia a kartografiakeyboard_arrow_downkeyboard_arrow_updoprava, pošty a telekomunikáciekeyboard_arrow_downkeyboard_arrow_upšpeciálne technické odborykeyboard_arrow_downkeyboard_arrow_uppoľnohospodársko-lesnícke a veterinárne vedy a náukykeyboard_arrow_downkeyboard_arrow_uppoľnohospodárstvo, lesné hospodárstvo a rozvoj vidieka IIkeyboard_arrow_downkeyboard_arrow_upspoločenské vedy, náuky a službykeyboard_arrow_downkeyboard_arrow_upekonomické vedykeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby Ikeyboard_arrow_downkeyboard_arrow_upekonomika a organizácia, obchod a služby IIkeyboard_arrow_downkeyboard_arrow_uppublicistika, knihovníctvo a vedecké informáciekeyboard_arrow_downkeyboard_arrow_uppredprimárne, základné a úplné stredné vzdelanie na gymnáziukeyboard_arrow_downkeyboard_arrow_upvedy a náuky o kultúre a umeníkeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba Ikeyboard_arrow_downkeyboard_arrow_upumenie a umeleckoremeselná tvorba IIkeyboard_arrow_downkeyboard_arrow_upUmenie a umeleckoremeselná tvorba  IIIkeyboard_arrow_downkeyboard_arrow_upvojenské a bezpečnostné vedy a náukykeyboard_arrow_downkeyboard_arrow_upbezpečnostné službykeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Ukončenie štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            maturitná skúškakeyboard_arrow_downkeyboard_arrow_upmaturitná skúška a výučný listkeyboard_arrow_downkeyboard_arrow_upvýučný listkeyboard_arrow_downkeyboard_arrow_upvyučenie a praktické školykeyboard_arrow_downkeyboard_arrow_upodborná prax alebo výcvikkeyboard_arrow_downkeyboard_arrow_upinékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Dĺžka štúdia
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            6,5 ročnékeyboard_arrow_downkeyboard_arrow_up9 ročnékeyboard_arrow_downkeyboard_arrow_up1 ročnékeyboard_arrow_downkeyboard_arrow_up4,5 ročnékeyboard_arrow_downkeyboard_arrow_up3 ročnékeyboard_arrow_downkeyboard_arrow_up2 ročnékeyboard_arrow_downkeyboard_arrow_up5,5 ročnékeyboard_arrow_downkeyboard_arrow_up4 ročnékeyboard_arrow_downkeyboard_arrow_up6 ročnékeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        

                        
                            

    Prijímacia skúška
    
        
            
                Vyberte...
            
            warning
            
                
                    keyboard_arrow_down
                
                
                    keyboard_arrow_up
                
            
        
        
        
            
                
            
            
                
                    
                        
                        Označiť všetky
                    
                
            
            Má skúšku zkeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upNemá skúšku zkeyboard_arrow_downkeyboard_arrow_upSlovenský jazyk a literatúrakeyboard_arrow_downkeyboard_arrow_upTalentová skúškakeyboard_arrow_downkeyboard_arrow_upMatematikakeyboard_arrow_downkeyboard_arrow_upDejepiskeyboard_arrow_downkeyboard_arrow_upChémiakeyboard_arrow_downkeyboard_arrow_upBiológiakeyboard_arrow_downkeyboard_arrow_upInékeyboard_arrow_downkeyboard_arrow_upAnglický jazykkeyboard_arrow_downkeyboard_arrow_upFrancúzsky jazykkeyboard_arrow_downkeyboard_arrow_upNemecký jazykkeyboard_arrow_downkeyboard_arrow_upRuský jazykkeyboard_arrow_downkeyboard_arrow_upNenachádza sa v číselníkukeyboard_arrow_downkeyboard_arrow_up
            
                
                    Zobraziť výsledky
                
            
            
                
                
                    Použiť
                
            
        
    
    
        
            Filtre:
            
        
        
            Filtre:
            
        
    

                        
                    
                    
                        
                            

    
    
    
        
            
                Talentový odbor
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Duálne vzdelávanie
                
            
            (nepovinné)
            
        
    

                        
                        
                            

    
    
    
        
            
                Bez prijímacej skúšky
                
            
            (nepovinné)
            
        
    

                        
                    
                
            

            
                
                    
                        Zobraziť rozšírený filter
                    
                    
                        keyboard_arrow_down
                    
                
                
                    
                        Skryť rozšírený filter
                    
                    
                        keyboard_arrow_up
                    
                
            
            
                Viac filtrov
                Menej filtrov
            

            
                Filtre
                
                    
                
                
                    Vymazať filtre
                
            
        

        
            +−

            
                
                
                    
                    
                    
                        
                            Stredné školy
                        
                        
                            1
                             škola zodpovedá vašim kritériám.
                        
                        
                            
                                
                                    
                                        
                                            filter_alt
                                        
                                        
                                            Zadajte kritéria vyhľadávania
                                        
                                    
                                    
                                        
                                            Zadajte kritéria vo vyhľadávaní, aby ste našli správne školy.
                                        
                                    
                                
                            
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        Pridať do obľúbených
                                                        Odstrániť z obľúbených
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            
                        
                            
                                1-1 z 1
                                
                                    chevron_left
                                    Predchádzajúca
                                
                                
                                    Ďalšia
                                    chevron_right
                                
                            
                        

                        
                        

                        

                        

                    

                    
                        
                            Obľúbené školy
                        
                        
                            0 obľúbených škôl
                            
                            
                        
                        
                        
                        
                            
                                
                                    
                                        bookmark_border
                                    
                                    
Žiadne obľúbené školy                                    
                                
                                
                                    
Zatiaľ nemáte obľúbené školy. Prejdite na kartu Všetky školy a pozrite si dostupné možnosti.                                    
                                
                            
                        
                    

                    
                        Stredné školy
                        
                            1
                            odbor ste pridali do prihlášky.
                        
                        
                                
                                    
                                        
                                            Gymnázium Metodova
                                        
                                        
                                            EDUID 
                                            910020859
                                        
                                        
                                            Metodova 2, 82108, Bratislava
                                            Zobraziť na mape
                                        
                                        
                                            link
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        school
                                                        gymnázium - Gymnázium
                                                    
                                                
                                                
                                                    
                                                        escalator_warning
                                                        súkromná
                                                    
                                                
                                                
                                                    
                                                        language
                                                        Vyučovací jazyk:slovenský a anglický bilingválny
                                                    
                                                
                                            
                                        

                                        
                                            Viac informácií o škole
                                            keyboard_arrow_down
                                        

                                        

                                            
                                                
                                                    
                                                        groups_2
                                                    
                                                    
                                                        
                                                            Vybraný pedagogický a odborný personál: -
                                                        
                                                    
                                                
                                                
                                                
                                            

                                            Zobraziť profil školy

                                            
                                                
                                                    Menej informácií o škole
                                                    keyboard_arrow_up
                                                
                                            
                                        
                                    

                                    
                                    
                                
                                    
                                        

                                            
                                                
                                                    
                                                        gymnázium
                                                        Netalentový odbor
                                                    
                                                    
                                                        Kód odboru: 7902J00
                                                         • 
                                                        Vyučovací jazyk: slovenský
                                                         • 
                                                        Kapacita odboru / voľná kapacita: 100 / -
                                                    
                                                    
                                                        Pridať do prihlášky
                                                        Odstrániť z prihlášky
                                                        
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            add
                                                        
                                                    
                                                
                                            
                                        
                                    
                                    
                                        
                                            
                                                
                                                    psychology
                                                    Netalentový odbor
                                                
                                            
                                            
                                                
                                                    call_split
                                                    Duálne vzdelávanie:
                                                    Zobraziť zoznam zamestnávateľov
                                                
                                            
                                            
                                                
                                                    calendar_month
                                                    4 ročné
                                                
                                            
                                            
                                                
                                                    sports_score
                                                    Ukončenie štúdia: maturitná skúška
                                                
                                            
                                            
                                                
                                                    today
                                                    
                                                
                                            
                                            
                                                
                                                    group
                                                    Vhodné pre žiakov: Žiaci 9. ročníka
                                                
                                            
                                            
                                                
                                                    content_paste
                                                    Prijímacia skúška: Slovenský jazyk a literatúra
                                                
                                            
                                            
                                                
                                                    rule
                                                    Zobraziť kritéria prijatia
                                                
                                            
                                        

                                        
                                            Zobraziť viac
                                            keyboard_arrow_down
                                        

                                        
                                            
                                                
                                                    Nie sú dostupné dáta o počte prijatých uchádzačov v minulom roku.
                                                    Nie sú dostupné dáta o predbežnom záujme o odbor.
                                                
                                            
                                            
                                                
                                                    Uplatnenie uchádzačov
                                                
                                                
                                                    Na základe dát inštitútu sociálnej politiky. Dáta sú platné k júnu roka nasledujúceho po absolvovaní štúdia.
                                                    
                                                        Zobraziť viac informácií o uplatnení
                                                    
                                                
                                                Nie sú dostupné dáta o uplatnení uchádzačov.
                                            
                                            
                                                Zobraziť menej
                                                keyboard_arrow_up
                                            
                                        
                                    
                                
                            
                                    
                                
                            

                        

                    
                
            
        
        
        
        
        


        
            
                
                    
                        
                            
                            
                        
                        
                            close
                        
                    
                    
                        
                            +−
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
        
            
                
                    
                        
                            Zoznam zamestnávateľov
                            
                        
                        
                            close
                        
                    
                    
                        
                            Škola v duálnom vzdelávaní spolupracuje so zoznamom zamestnávateľov, ktorý možno rozšíriť podľa záujmu žiaka.
                            
                                Kapacita pre duálne vzdelávanie: 
                                Predpokladaný počet žiakov, ktorých bude možné zaradiť do duálneho vzdelávania.
                            
                        
                        
                    
                    
                        Zavrieť
                    
                
            
            
        
    






        
    

    
        
            
                Doplňte informácie k odboru.
            
        

        
        

        
            Stredné školy
            
                1
                školu ste pridali do prihlášky.
            
        

        
            
                1
                Vami zvolené poradie školy
                
                    arrow_upward
                    Presunúť vyššie
                
                
                    Presunúť nižšie
                    arrow_downward
                
                drag_indicator
            
            
                
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
                Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
            
            (nepovinné)
            Teoretické vyučovanie kombinované s praktickou výučbou vo firmách.
        
    

                    
                    
                        

    
    
    
        
            
                Záujem o školský internát
                
            
            (nepovinné)
            
        
    

                    
                    
                        

    
        Termín prijímacej skúšky
        (nepovinné)
    
    
    
        1. termín (1.kolo)1. termín (2.kolo)2. termín (1.kolo)
        warning
        warning
        
            keyboard_arrow_down
        
    
    
        Filtre:
        
            
            
                close
            
        
    

                    
                

                Odstrániť z prihlášky
            

        
    

    

    

    


                    
                    
                        


    const pageTextsZakonnyZastupca = {
        &quot; , &quot;'&quot; , &quot;8042&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Nastala neočakávaná chyba, zopakujte operáciu alebo kontaktujte  technickú podporu.&quot; , &quot;'&quot; , &quot;,
        dietata: &quot;dieťaťa&quot;,
        ziaka: &quot;žiaka&quot;
    };

    const controlSettingsZastupca = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,

        zastupca1RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },

        zastupca1Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },

        zastupca1AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        zastupca2Radio: {
            label: &quot; , &quot;'&quot; , &quot;Vyberte jednu z možností&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot;rfo&quot;,
                    value: &quot;RFO&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Iný zákonný zástupca&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Vyplňte v prípade, ak druhým zákonným zástupcom je iná osoba.&lt;/div>&lt;/div>&quot;,
                    value: &quot;INY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca je známy&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Vyplňte údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;ZNAMY&quot;
                },
                {
                    label: &quot;&lt;div>&lt;div>Druhý zákonný zástupca nie je známy&lt;/div>&lt;div class=&quot; , &quot;'&quot; , &quot;govuk-hint&quot; , &quot;'&quot; , &quot;>Túto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu.&lt;/div>&lt;/div>&quot;,
                    value: &quot;NEZNAMY&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        zastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        zastupca2RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        zastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        zastupca2NemaRodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Osoba nemá pridelené rodné číslo&quot; , &quot;'&quot; , &quot;,
            required: false
        },
        zastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        zastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;E-mail&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        zastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        zastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        // rfo zastupca 2
        rfoZastupca2Meno: {
            label: &quot; , &quot;'&quot; , &quot;Meno&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte krstné meno osoby, ak má osoba viacero krstných mien, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2Priezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: true
        },
        rfoZastupca2RodnePriezvisko: {
            label: &quot; , &quot;'&quot; , &quot;Rodné priezvisko&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte priezvisko osoby, ak má osoba viacero priezvisk, oddeľte ich medzerou.&quot;,
            required: false
        },
        rfoZastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        rfoZastupca2DatumNarodenia: {
            label: &quot; , &quot;'&quot; , &quot;Dátum narodenia&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Zadajte dátum narodenia v tvare DD.MM.RRRR (napr. 10.08.2000). &quot;,
            required: true
        },
        rfoZastupca2Email: {
            label: &quot; , &quot;'&quot; , &quot;E-mail&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Email má neplatný formát. Uveďte emailovú adresu v tvare napr. nazov@mail.sk&quot;,
            required: true
        },
        rfoZastupca2Telefon: {
            label: &quot; , &quot;'&quot; , &quot;Telefónne číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte telefónne číslo vo formáte s predvoľbou, napr. +421 9XX XXX XXX&quot; , &quot;'&quot; , &quot;,
            regexError: &quot;Položka Telefón má neplatný formát, uveďte číslo v medzinárodnom formáte (napr. pre Slovensko +421XXXXXXXXX).&quot;,
            defaultValue: &quot; , &quot;'&quot; , &quot;+421&quot; , &quot;'&quot; , &quot;,
            required: true
        },
        rfoZastupca2AdresaRadio: {
            label: &quot; , &quot;'&quot; , &quot;Uveďte adresu, na ktorú prijímate poštové zásielky&quot; , &quot;'&quot; , &quot;,
            options: [
                {
                    label: &quot; , &quot;'&quot; , &quot;window.dtc.form.zastupcovia.zakonnyZastupca1.adresaVyskladana&quot; , &quot;'&quot; , &quot;,
                    value: &quot;EXISTUJUCA&quot;
                },
                {
                    label: &quot;Iná korešpondenčná adresa&quot;,
                    value: &quot;INA&quot;
                }
            ],
            direction: &quot; , &quot;'&quot; , &quot;column&quot; , &quot;'&quot; , &quot;,
        },

        agreementRadio: {
            label: &quot; , &quot;'&quot; , &quot;Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca {osoba}.&quot; , &quot;'&quot; , &quot;,
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

        komunikaciaLenSZZ1: {
            label: &quot; , &quot;'&quot; , &quot;Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).&quot; , &quot;'&quot; , &quot;,
            required: false
        },
        noAgreementReason: {
            label: &quot; , &quot;'&quot; , &quot;Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: &quot; , &quot;'&quot; , &quot;,
            required: true
        },

        DataCheckWarningBox: {
            headerText: &quot; , &quot;'&quot; , &quot;Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty&quot; , &quot;'&quot; , &quot;,
            contentSelector: &quot; , &quot;'&quot; , &quot;#dataCheckWarningBoxContent&quot; , &quot;'&quot; , &quot;
        }

    };





    
        Zákonný zástupca žiaka
        
            
                Skontrolujte a doplňte údaje o zákonných zástupcoch. V prípade potreby opravte nesprávne informácie.
            
        
    

    Polia označené hviezdičkou sú povinné

    
        
            Osobné údaje zákonného zástupcu č.1
        
        
            
                
                    Údaje z vášho profilu
                
                Upraviť
            
            
                
                    Meno
                    Stanislav
                
                
                    Priezvisko
                    Štefánek
                
                
                    Rodné priezvisko
                    -
                
                
                    Rodné číslo
                    030724/6027
                
                
                    Dátum narodenia
                    24.07.2003
                
                
                    Kontaktný e-mail
                    barcik@ditec.sk
                
            
        

        
            

    
        Rodné priezvisko
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
                        Iná korešpondenčná adresa 
                         
                

    
        

    
        Krajina
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


    

    
        

    
        Adresa
        (nepovinné)
    

    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.

    
        
    


    

    
        

    
        Adresa
        (nepovinné)
        
    
    
        warning
        
        
            0/100
        
    
    Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.


    



    window[&quot; , &quot;'&quot; , &quot;zastupca1InaAdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
    zastupca1InaAdresaKrajina: {
        label: &quot; , &quot;'&quot; , &quot;Krajina&quot; , &quot;'&quot; , &quot;,
            required: true,
                attributes: { maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot; },
        validators: [
            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
            { type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Vyhľadajte krajinu po zadaní prvých 3 znakov a vyberte krajinu zo zoznamu zobrazených krajín!&quot; , &quot;'&quot; , &quot; }
        ]
    },

    zastupca1InaAdresaAdresaRA: {
        label: &quot; , &quot;'&quot; , &quot;Adresa&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot; , &quot;'&quot; , &quot;,
                required: true,
                    placeholder: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                        validators: [
                            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
                            // ak user niečo napíše, musí to aj vybrať zo zoznamu
                            { type: &quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; }
                        ]
    },

    zastupca1InaAdresaAdresa: {
        label: &quot;Adresa&quot;,
            hint: &quot;Zadajte kompletné údaje o adrese: ulica, číslo domu, poštové smerovacie číslo, mesto a prípadne štát alebo provinciu.&quot;,
                required: true,
                    attributes: { maxLength: &quot; , &quot;'&quot; , &quot;100&quot; , &quot;'&quot; , &quot; },
        validators: [
            { type: &quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, message: &quot; , &quot;'&quot; , &quot;Toto pole je povinné.&quot; , &quot;'&quot; , &quot; },
            {
                type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                functionOrRegex: /^$|^[Á-Žá-žA-Za-z0-9\,\.\/\&quot; , &quot;'&quot; , &quot;\-\s]{1,100}$/,
                message: &quot;Zadajte zahraničnú adresu.&quot;
            }
        ]
    }
        };


            
    

            
            
        
    

    

    


        
            
                Osobné údaje zákonného zástupcu č.2
            
            
                

    
    
        Vyberte jednu z možností
        (nepovinné)
    
    
    
    
                        
                        Druhý zákonný zástupca je známyVyplňte údaje druhého zákonného zástupcu. 
                         
            
                

    
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
            
        
    




    window[&quot; , &quot;'&quot; , &quot;zastupca2RodneCisloControlSettings&quot; , &quot;'&quot; , &quot;] = {
        zastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            attributes: {
                 minLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;,
                 maxLength: &quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: &quot; , &quot;'&quot; , &quot;Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).&quot; , &quot;'&quot; , &quot;
                },
            ],

            validationMessageDelitelnost11: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.&quot; , &quot;'&quot; , &quot;,
            validationMessage9Miestne: &quot; , &quot;'&quot; , &quot;9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.&quot; , &quot;'&quot; , &quot;,
            validationMessageTretiZnak: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.&quot; , &quot;'&quot; , &quot;,

        },
    };





            

    
    
    
        
            
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


            
    

            
            
        
                        
                        Druhý zákonný zástupca nie je známyTúto možnosť vyberte v prípade, ak druhý zákonný zástupca dieťaťa neexistuje napr. z dôvodu úmrtia alebo jeho neuvedenia na rodnom liste dieťaťa. Táto možnosť neplatí pre prípady, keď bol druhému rodičovi obmedzený alebo pozastavený výkon rodičovských práv - vtedy je nevyhnutné jeho údaje vyplniť ako údaje druhého zákonného zástupcu. 
                         
    

            
            
                

    
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
            
        
    




    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2RodneCisloControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2RodneCislo: {
            label: &quot; , &quot;'&quot; , &quot;Rodné číslo&quot; , &quot;'&quot; , &quot;,
            hint: &quot; , &quot;'&quot; , &quot;Zadajte vo formáte XXXXXX/XXXX&quot; , &quot;'&quot; , &quot;,
            attributes: {
                 minLength: &quot; , &quot;'&quot; , &quot;10&quot; , &quot;'&quot; , &quot;,
                 maxLength: &quot; , &quot;'&quot; , &quot;11&quot; , &quot;'&quot; , &quot;,
            },
            validators: [
                {
                    type: &quot; , &quot;'&quot; , &quot;regex&quot; , &quot;'&quot; , &quot;,
                    functionOrRegex: /^$|^[0-9]{6}[\/][0-9]{3,4}$/,
                    message: &quot; , &quot;'&quot; , &quot;Rodné číslo musí byť v tvare s lomkou 6 číslic \&quot;/\&quot; a 3 alebo 4 celočíselné znaky (napr.: 120610/6605).&quot; , &quot;'&quot; , &quot;
                },
            ],

            validationMessageDelitelnost11: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, rodné číslo musí byť deliteľné 11.&quot; , &quot;'&quot; , &quot;,
            validationMessage9Miestne: &quot; , &quot;'&quot; , &quot;9-miestne rodné čísla platia pre osoby narodené pred rokom 1.1.1954.&quot; , &quot;'&quot; , &quot;,
            validationMessageTretiZnak: &quot; , &quot;'&quot; , &quot;Zadali ste neplatné rodné číslo, tretí znak rodného čísla musí byť \&quot;0\&quot;, \&quot;1\&quot;, \&quot;5\&quot; alebo \&quot;6\&quot;.&quot; , &quot;'&quot; , &quot;,

        },
    };





            

    
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
    



    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2DatumNarodeniaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2DatumNarodeniaDen: {
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
        rfoZastupca2DatumNarodeniaMesiac: {
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
        rfoZastupca2DatumNarodeniaRok: {
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



            
                

    
        E-mail
        (nepovinné)
    
    Na uvedenú e-mailovú adresu bude zákonnému zástupcovi doručená notifikácia.
    
        
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
            
                

    
    
        Uveďte adresu, na ktorú prijímate poštové zásielky
        (nepovinné)
    
    
    
    
                        
                        Kladnianska 58/98, 01508, Kriváň (Detva), Slovenská republika 
                         
                        
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


    



    window[&quot; , &quot;'&quot; , &quot;rfoZastupca2AdresaControlSettings&quot; , &quot;'&quot; , &quot;] = {
        rfoZastupca2AdresaKrajina: {
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
        rfoZastupca2AdresaObec: {
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
        rfoZastupca2AdresaUlica: {
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
        rfoZastupca2AdresaUlicaReq: {
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
        rfoZastupca2AdresaSupisneCislo: {
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
        rfoZastupca2AdresaOrientacneCislo: {
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
        rfoZastupca2AdresaPSC: {
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
        rfoZastupca2AdresaAdresa: {
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


            
    

            
            
        
        

        
            error

    Škola môže na overenie správnosti údajov požadovať nasledujúce dokumenty
    
        
            
            
            
                rodný list dieťaťa (na nahliadnutie),
                úmrtný list druhého rodiča (na nahliadnutie),
                neoverenú kópiu súdneho rozhodnutia (môžete ju nahrať ako prílohu v nasledujúcich krokoch alebo doniesť na zápis).
            
        
        
        
            
            
        
    


        

        

        

        
    




    Zákonný zástupca žiaka
    
        
            Súhlas druhého zákonného zástupcu s podaním prihlášky.
        
    

    
        info
        
            Podľa zákona č. 245/2008 Z. z. o výchove a vzdelávaní (školský zákon) a o zmene a doplnení niektorých zákonov v znení neskorších predpisov sa na prihláške o prijatie vyžaduje podpis obidvoch zákonných zástupcov dieťaťa. Ak nie je možné získať podpis druhého zákonného zástupcu, je potrebné uviesť dôvod a priložiť potrebný dokument ako prílohu.Viac informácií
        
    

    Polia označené hviezdičkou sú povinné

    
        
            

    
    
        Čestne vyhlasujem, že s podaním tejto prihlášky súhlasí aj druhý zákonný zástupca žiaka.
        (nepovinné)
    
    
    
    
                        
                        áno 
                         
            

    
    
    
        
            
                Týmto žiadam, aby ste vo veciach súvisiacich s prijímacím konaním môjho dieťaťa komunikovali výhradne so mnou ako so zákonným zástupcom.
                Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
            
            (nepovinné)
            Beriem na vedomie, že k tomuto účelu je potrebné doložiť podpísané vyhlásenie oboch zákonných zástupcov podľa § 144a ods. 4 zákona č. 245/2008 Z. z. (školský zákon).
        
    

        
                        
                        nie 
                         
    

        
            
                

    
        Dôvod, prečo nebolo dané čestné prehlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky: 
        (nepovinné)
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            
            
                error
                
                    Doručenie prílohy je zákonnou povinnosťou. Zatiaľ ju nemusíte priložiť – vyžiadame ju neskôr
                    
                        Rozhodnutie súdu — ak bol druhému zákonnému zástupcovi obmedzený alebo pozastavený výkon rodičovských práv a povinností, je potrebné priložiť kópiu tohto rozhodnutia.
                        Potvrdenie od lekára — ak druhý zákonný zástupca nie je schopný podpísať sa zo zdravotných dôvodov, je toto potvrdenie potrebné.
                        Čestné vyhlásenie zákonného zástupcu — ak získanie podpisu druhého zákonného zástupcu je spojené s ťažko prekonateľnou prekážkou a je to v najlepšom záujme dieťaťa.
                    
                
            

        

        
        
    


                    
                    
                        

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
            hint: &quot;Uveďte posledný ukončený ročník základnej školy.&quot;,
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
        }
    };

    const controlSettingsInformacieOZakladnejSkole = {
        povinneError: &quot;Toto pole je povinné.&quot;,
        nepovinne: &quot;&quot;,
        zoZSNaSlovensku: &quot;Zo ZŠ na Slovensku&quot;,
        zoZSVZahranici: &quot;Zo školy v zahraničí&quot;
    };




    Informácie o základnej škole
    
        
            Vyplňte informácie o základnej škole, ktorú žiak aktuálne navštevuje.
        
    

    
        
            
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
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        
        warning
        
            keyboard_arrow_down
        
    


            

        
                        
                        Zo školy v zahraničí 
                         
    

        

        

        

            
                

    
        Ročník
        (nepovinné)
    
    Uveďte posledný ukončený ročník základnej školy.
    
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
    
    
    
        
        poľský
        warning
        
            keyboard_arrow_down
        
    
anglickýbulharskýčeskýfrancúzskymaďarskýnemeckýnesleduje sapoľskýrómskyrusínskyruskýslovenskýslovenský - maďarskýslovenský - nemeckýslovenský - rómskyslovenský - rusínskyslovenský - ukrajinskýslovenský a anglický bilingválnyslovenský a anglický s medzinár. programomslovenský a čínsky bilingválnyslovenský a francúzsky bilingválnyslovenský a francúzsky s medzinár. programomslovenský a iný bilingválnyslovenský a iný s medzinár. programomslovenský a nemecký bilingválnyslovenský a nemecký s medzinár. programomslovenský a ruský bilingválnyslovenský a ruský s medzinár. programomslovenský a španielsky bilingválnyslovenský a španielsky s medzinár. programomslovenský a taliansky bilingválnyslovenský a taliansky s medzinár. programomšpanielskytalianskyukrajinský

            

        
    

    
        
            
                
                    
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
        zmeniliSteUdajePanelTitle: &quot;Zmenili ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        zmeniliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť nasledovné:&lt;ul>&lt;/ul>&quot;,
        neuviedliSteUdajePanelTitle: &quot;Neuviedli ste údaje pri jednom alebo viacerých predmetoch, preto bude potrebné ich overenie&quot;,
        neuviedliSteUdajePanelText: &quot;Ak je žiak žiakom základnej školy na Slovensku, známky potvrdí riaditeľ školy. V opačnom prípade bude potrebné doložiť overené kópie vysvedčení.&quot;,
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
            label: &quot;Mám známky v inej známkovacej schéme&quot;,
            hint: &quot;Označte, ak vaše známky nie sú v bežnej slovenskej škále (1 – 5 alebo výborný, chválitebný, dobrý, dostatočný, nedostatočný).&quot;,
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

    
        Za posledný ročník uveďte známky z polročného vysvedčenia. Ak žiak opakoval ročník, uveďte známky iba za ten ročník, v ktorom prospel.
        Skontrolujte, či všetky známky z požadovaných predmetov za posledné 4 ročníky základnej školy zodpovedajú údajom na vysvedčeniach. Ak žiak niektorý ročník opakoval, zapíšte známky z posledného absolvovaného ročníka. Chýbajúce alebo nesprávne známky  doplňte alebo opravte.
    

    
        Ak používate EduPage, môžete si známky jednoducho načítať priamo odtiaľ.
        Pridať známky z EduPage
    

    
        Známky v inej známkovacej schéme
        
            

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        
    

    
        
            
                
                    
                    
                    
                    
                    
                    
                    
                    
                
            
        
        
        
            
                
                    
                    
                    
                
                
                    
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

    
        Uveďte iba súťaže, v ktorých žiak dosiahol významné umiestnenie alebo výsledok. Súťaže, za ktoré sa prideľujú body pri prijímacom konaní, nájdete v kritériách školy, na ktorú sa žiak hlási.
    

    
        
            
                
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
        spravaRozhodnutie: &quot;Rozhodnutie:&quot;,
        spravaDoplnenePodklady: &quot;Doplnené podklady:&quot;,
        spravaRiaditel: &quot;Riaditeľ&quot;,
        spravaDovod: &quot;Dôvod:&quot;,
        spravaPotvrditNastup: &quot;Potvrdiť nástup&quot;,
        spravaPridatPrilohu: &quot;Pridať prílohu&quot;,
        spravaPrilozeneDokumenty: &quot;Priložené dokumenty:&quot;,
        nastupPotvrdeny: &quot;nástup potvrdený&quot;,
        nenastupi: &quot;nenastúpi&quot;,
        identifikator: &quot;Identifikátor&quot;,
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
        stavPrihlasky: &quot;Stav prihlášky&quot;,
        podana: &quot;Podaná&quot;,
        zastupcaSuhlas: &quot;Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}&quot;,
        elektronicky: &quot;Elektronicky&quot;,
        prichodZoZahraniciaPDF: &quot;Zo zahraničia&quot;,
        ziadneSutaze: &quot;Neboli nahrané žiadne súťaže.&quot;,

        radoveCislovky:
        { 
            &quot;1&quot;: &quot;Prvý&quot;,
            &quot;2&quot;: &quot;Druhý&quot;,
            &quot;3&quot;: &quot;Tretí&quot;,
            &quot;4&quot;: &quot;Štvrtý&quot;,
            &quot;5&quot;: &quot;Piaty&quot;,
            &quot;6&quot;: &quot;Šiesty&quot;,
            &quot;7&quot;: &quot;Siedmy&quot;,
            &quot;8&quot;: &quot;Ôsmy&quot;,
            &quot;9&quot;: &quot;Deviaty&quot;,
            &quot;10&quot;: &quot;Desiaty&quot;
        },

        cestnePrehlasenie: {
            label: &quot;Čestné vyhlásenie&quot;,
            hint: &quot;Čestne vyhlasujem, že zákonný zástupca/zákonní zástupcovia a žiak, potvrdzujú správnosť a pravdivosť údajov v prihláške.&quot;,
            required: true,
            showRequiredHint: true,
        },
        suhlasOsobneUdaje: {
            label: &quot;Súhlas so spracúvaním osobných údajov&quot;,
            hint: &quot;&lt;div>Súhlasím so spracúvaním osobných údajov v rozsahu údajov uvedených v prihláške na účel podania prihlášky. &lt;a href=&quot; , &quot;'&quot; , &quot;/Ochrana-osobnych-udajov&quot; , &quot;'&quot; , &quot;>Viac informácií nájdete tu.&lt;/a>&lt;/div>&quot;,
            required: true,
            showRequiredHint: true,
        },
        dorucenie: {
            label: &quot;Napriek tomu požadujem aj doručenie poštou alebo do elektronickej schránky.&quot;,
            hint: &quot;&lt;div>Listová zásielka bude doručená na príslušnú korešpondenčnú adresu alebo do elektronickej schránky na portáli  &lt;a href=&quot; , &quot;'&quot; , &quot;https://www.slovensko.sk/&quot; , &quot;'&quot; , &quot;>slovensko.sk&lt;/a>.&lt;/div>&quot;,
            required: false
        }
    };




    Súhrnný prehľad
    
        
            Dôkladne skontrolujte všetky informácie. Urýchlite tak prijímací proces.
        
    

    Polia označené hviezdičkou sú povinné

    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
            Upraviť
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
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
        
        
            Neboli nahrané žiadne súťaže.
        
    

    

    
        
            
                Prílohy
            
            Upraviť
        
        
            

            
        
    

    
        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

        

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    


        
            
                
                    
                
                
                    Rozhodnutia o prijatí budú zverejnené na elektronickej výveske, o čom budete informovaný e-mailovou správou.
                    

    
    
    
        
            
                
                
            
            (nepovinné)
            
        
    

                
            
        
    

    
    




    Detail prihlášky


    
        Všeobecné informácie
        
            
                Identifikátor prihlášky
                -
            
            
                Školský rok
                -
            
            
                Dátum podania
                -
            
            
                Spôsob podania
                -
            
            
                Kolo prijímacieho konania
                -
            
            
                Prístupový kód
                -
            
        
    

    
        
            
                Osobné údaje žiaka
            
        
        
            
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
            
        
    

    
        
            
                Doplňujúce informácie o {osoba}
            
        
        
            
                
                    
                        Žiadam o prijatie dieťaťa na
                        -
                    
                    
                        Požadovaná výchova
                        -
                    
                    
                        Záujem o stravovanie v jedálni
                        -
                    
                    
                        Záujem o školský klub detí
                        -
                    
                    
                        Zdravotné znevýhodnenie dieťaťa / dieťa s nadaním
                        -
                    
                    
                        Popis znevýhodnenia / nadania
                        -
                    
                    
                        Poznámka
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
            
        
        
        
    

    
        
            
                Osobné údaje zákonných zástupcov žiaka
            
        
        
            Osobné údaje zákonného zástupcu č. 1
            
                
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
                
                
                    Dátum narodenia
                    -
                
                
                    Korešpondenčná adresa
                    -
                
                
                    E-mail
                    -
                
                
                    Telefónne číslo
                    -
                
                
                    Čestne vyhlasujem, že s podaním prihlášky súhlasí aj druhý zákonný zástupca {osoba}
                    -
                
                
                    Dôvod, prečo nebolo dané čestné vyhlásenie o súhlase druhého zákonného zástupcu s podaním prihlášky
                    -
                
            

            Druhý zákonný zástupca nie je známy.
        
    

    
        
            
                Informácie o základnej škole
            
        
        
            
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
            
        
        
        
    

    
        
            
                Súťaže
            
        
        
        
    

    

    
        
            
                Prílohy
            
        
        
            
            
        
    
    
    


EXPORT PDF
                    

                    
                        Späť
                        
                            Uložiť a odísť
                            Ďalej
                            Odoslať prihlášku
                                Stiahnuť XML
                            
                                delete_outline
                                Odstrániť
                            
                            Aktualizovať a odísť
                        
                    
                &quot;))]</value>
      <webElementGuid>d45a36c7-c4a5-4eeb-8ee2-42a033b3b7fc</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
