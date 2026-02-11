<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_K2 Kritrium 2_modal-content</name>
   <tag></tag>
   <elementGuidId>7f00be6c-f8d8-4896-ae34-847aee794465</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='sub-riaditel-kriteria']/div[3]/div/div/div[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.modal.kriteria-vypocet-modal > div.modal-inner > div.modal-content</value>
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
      <webElementGuid>4bd5215d-7ac5-42e4-91c3-f7d0c2d36397</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal-content</value>
      <webElementGuid>c738dd5f-8db8-44e2-8b1a-c54bc86f8428</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
                    warning_rounded
                    
                        Neplatné znaky vo výpočte
                        Vo výpočte je použitý neplatný znak. Skontrolujte prosím výraz a opravte chybu. Všetky povolené znaky nájdete v návode.
                        Zobraziť návod
                    
                
                Polia označené hviezdičkou sú povinné
                
                    

    
        Výpočet
        (nepovinné)
        
    Vložiť funkciu:Naj()Priemer()Zaokr()Min()Striktný_priemer()Max()
    
        warning
        
        
            9/100
        
    Priemer(50+4)
    


                
                Použitie premenných: Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.
                
                    

        
            
                

                    
                        
                            
                                Ako vyplniť Výpočet?
                            
                        
                        
                            
                                
                                    add
                                
                            
                        
                    
                
            
            
                    
Výpočet:
Pri zadávaní výpočtu môžete využívať rôzne funkcie, operátory a premenné:
Dostupné funkcie:

	Naj() – najlepšia známka spomedzi vymenovaných, napríklad naj(CHE9, FYZ9, BIO9),
	Priemer() – priemer vymenovaných hodnôt, napríklad priemer(ANJ9, NEJ9, RUJ9), funguje aj vtedy, ak sú zadané len dve z troch hodnôt,
	Zaokr() – zaokrúhlenie desatinného čísla na celé číslo,
	Min() – najnižšia hodnota spomedzi vymenovaných hodnôt,
	Striktný_priemer() – priemer vymenovaných hodnôt, ktorý vyžaduje, aby boli všetky hodnoty zadané, napríklad striktný_priemer(MAT9, MAT8), vyžaduje všetky hodnoty zadané, inak upozorní,
	Max() –najvyššia hodnota spomedzi vymenovaných hodnôt.

Základné operátory:

	aritmetické: + (sčítanie), − (odčítanie), × (násobenie), / (delenie),
	porovnávacie: &lt; (menšie), > (väčšie), = (rovné), ≥ alebo >= (väčšie alebo rovné), ≤ alebo &lt;= (menšie alebo rovné),
	logické: &amp;, ∧, a (logické „a zároveň“), ∨ (logické „alebo“),
	skupinovacie: ( ) (zátvorky pre sprehľadnenie a určenie poradia výpočtov).

Použitie premenných:
Premenné pre hodnotenia z testovania:

	T9SJL – hodnotenie Testovania 9 zo slovenského jazyka a literatúry,
	T9MAT – hodnotenie Testovania 9 z matematiky,
	T9SJS – hodnotenie Testovania 9 zo slovenského jazyka a slovenskej literatúry,
	T9MJL – hodnotenie Testovania 9 z maďarského jazyka a literatúry,
	T9MJG – hodnotenie Testovania 9 z maďarského jazyka,
	T9SK – hodnotenie Testovania 9 zo slovenského jazyka a slovenského jazyka a slovenskej literatúry – zjednodušenie pre ďalšie spracovanie.


Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.


SJLPredmetOficiálna trojpísmenková skratka.

9RočníkČíslom, alebo mx (m1 = rok do minulosti)
iPolroki alebo ii, ak sa neuvedie, ide o posledný, z ktorého je udelená známka
1Známka
hodnota známky (vráti 1, ak taká známka bola, inak 0), alebo ak sa tento parameter neuvedie, vráti hodnotu udelenej známky

Napríklad Naj(SJL9i, SJL8i)

Zobraziť návod
            
        


                
            </value>
      <webElementGuid>1015244b-3933-4f25-925d-fca6fe90125f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;sub-riaditel-kriteria&quot;)/div[@class=&quot;modal-wrapper kriteria-vypocet-modal-wrapper&quot;]/div[@class=&quot;modal kriteria-vypocet-modal&quot;]/div[@class=&quot;modal-inner&quot;]/div[@class=&quot;modal-content&quot;]</value>
      <webElementGuid>3a2ee873-762b-4c0b-8998-001ad444253f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='sub-riaditel-kriteria']/div[3]/div/div/div[2]</value>
      <webElementGuid>f8146e46-9ebd-488a-8b28-3b20d8d648f0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='K2: Kritérium 2'])[1]/following::div[1]</value>
      <webElementGuid>ecdacb2c-c275-4172-ae2c-2d291edae256</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Automatický výpočet'])[1]/following::div[2]</value>
      <webElementGuid>670152a5-cb5b-4863-b35f-f377cbbff667</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[4]/div[3]/div/div/div[2]</value>
      <webElementGuid>d7246ddc-fd80-400f-affa-018188a785ea</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = '
                
                    warning_rounded
                    
                        Neplatné znaky vo výpočte
                        Vo výpočte je použitý neplatný znak. Skontrolujte prosím výraz a opravte chybu. Všetky povolené znaky nájdete v návode.
                        Zobraziť návod
                    
                
                Polia označené hviezdičkou sú povinné
                
                    

    
        Výpočet
        (nepovinné)
        
    Vložiť funkciu:Naj()Priemer()Zaokr()Min()Striktný_priemer()Max()
    
        warning
        
        
            9/100
        
    Priemer(50+4)
    


                
                Použitie premenných: Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.
                
                    

        
            
                

                    
                        
                            
                                Ako vyplniť Výpočet?
                            
                        
                        
                            
                                
                                    add
                                
                            
                        
                    
                
            
            
                    
Výpočet:
Pri zadávaní výpočtu môžete využívať rôzne funkcie, operátory a premenné:
Dostupné funkcie:

	Naj() – najlepšia známka spomedzi vymenovaných, napríklad naj(CHE9, FYZ9, BIO9),
	Priemer() – priemer vymenovaných hodnôt, napríklad priemer(ANJ9, NEJ9, RUJ9), funguje aj vtedy, ak sú zadané len dve z troch hodnôt,
	Zaokr() – zaokrúhlenie desatinného čísla na celé číslo,
	Min() – najnižšia hodnota spomedzi vymenovaných hodnôt,
	Striktný_priemer() – priemer vymenovaných hodnôt, ktorý vyžaduje, aby boli všetky hodnoty zadané, napríklad striktný_priemer(MAT9, MAT8), vyžaduje všetky hodnoty zadané, inak upozorní,
	Max() –najvyššia hodnota spomedzi vymenovaných hodnôt.

Základné operátory:

	aritmetické: + (sčítanie), − (odčítanie), × (násobenie), / (delenie),
	porovnávacie: &lt; (menšie), > (väčšie), = (rovné), ≥ alebo >= (väčšie alebo rovné), ≤ alebo &lt;= (menšie alebo rovné),
	logické: &amp;, ∧, a (logické „a zároveň“), ∨ (logické „alebo“),
	skupinovacie: ( ) (zátvorky pre sprehľadnenie a určenie poradia výpočtov).

Použitie premenných:
Premenné pre hodnotenia z testovania:

	T9SJL – hodnotenie Testovania 9 zo slovenského jazyka a literatúry,
	T9MAT – hodnotenie Testovania 9 z matematiky,
	T9SJS – hodnotenie Testovania 9 zo slovenského jazyka a slovenskej literatúry,
	T9MJL – hodnotenie Testovania 9 z maďarského jazyka a literatúry,
	T9MJG – hodnotenie Testovania 9 z maďarského jazyka,
	T9SK – hodnotenie Testovania 9 zo slovenského jazyka a slovenského jazyka a slovenskej literatúry – zjednodušenie pre ďalšie spracovanie.


Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.


SJLPredmetOficiálna trojpísmenková skratka.

9RočníkČíslom, alebo mx (m1 = rok do minulosti)
iPolroki alebo ii, ak sa neuvedie, ide o posledný, z ktorého je udelená známka
1Známka
hodnota známky (vráti 1, ak taká známka bola, inak 0), alebo ak sa tento parameter neuvedie, vráti hodnotu udelenej známky

Napríklad Naj(SJL9i, SJL8i)

Zobraziť návod
            
        


                
            ' or . = '
                
                    warning_rounded
                    
                        Neplatné znaky vo výpočte
                        Vo výpočte je použitý neplatný znak. Skontrolujte prosím výraz a opravte chybu. Všetky povolené znaky nájdete v návode.
                        Zobraziť návod
                    
                
                Polia označené hviezdičkou sú povinné
                
                    

    
        Výpočet
        (nepovinné)
        
    Vložiť funkciu:Naj()Priemer()Zaokr()Min()Striktný_priemer()Max()
    
        warning
        
        
            9/100
        
    Priemer(50+4)
    


                
                Použitie premenných: Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.
                
                    

        
            
                

                    
                        
                            
                                Ako vyplniť Výpočet?
                            
                        
                        
                            
                                
                                    add
                                
                            
                        
                    
                
            
            
                    
Výpočet:
Pri zadávaní výpočtu môžete využívať rôzne funkcie, operátory a premenné:
Dostupné funkcie:

	Naj() – najlepšia známka spomedzi vymenovaných, napríklad naj(CHE9, FYZ9, BIO9),
	Priemer() – priemer vymenovaných hodnôt, napríklad priemer(ANJ9, NEJ9, RUJ9), funguje aj vtedy, ak sú zadané len dve z troch hodnôt,
	Zaokr() – zaokrúhlenie desatinného čísla na celé číslo,
	Min() – najnižšia hodnota spomedzi vymenovaných hodnôt,
	Striktný_priemer() – priemer vymenovaných hodnôt, ktorý vyžaduje, aby boli všetky hodnoty zadané, napríklad striktný_priemer(MAT9, MAT8), vyžaduje všetky hodnoty zadané, inak upozorní,
	Max() –najvyššia hodnota spomedzi vymenovaných hodnôt.

Základné operátory:

	aritmetické: + (sčítanie), − (odčítanie), × (násobenie), / (delenie),
	porovnávacie: &lt; (menšie), > (väčšie), = (rovné), ≥ alebo >= (väčšie alebo rovné), ≤ alebo &lt;= (menšie alebo rovné),
	logické: &amp;, ∧, a (logické „a zároveň“), ∨ (logické „alebo“),
	skupinovacie: ( ) (zátvorky pre sprehľadnenie a určenie poradia výpočtov).

Použitie premenných:
Premenné pre hodnotenia z testovania:

	T9SJL – hodnotenie Testovania 9 zo slovenského jazyka a literatúry,
	T9MAT – hodnotenie Testovania 9 z matematiky,
	T9SJS – hodnotenie Testovania 9 zo slovenského jazyka a slovenskej literatúry,
	T9MJL – hodnotenie Testovania 9 z maďarského jazyka a literatúry,
	T9MJG – hodnotenie Testovania 9 z maďarského jazyka,
	T9SK – hodnotenie Testovania 9 zo slovenského jazyka a slovenského jazyka a slovenskej literatúry – zjednodušenie pre ďalšie spracovanie.


Pri výpočte môžete dosadzovať premenné, ktoré odkazujú na jednotlivé známky alebo hodnotenia.


SJLPredmetOficiálna trojpísmenková skratka.

9RočníkČíslom, alebo mx (m1 = rok do minulosti)
iPolroki alebo ii, ak sa neuvedie, ide o posledný, z ktorého je udelená známka
1Známka
hodnota známky (vráti 1, ak taká známka bola, inak 0), alebo ak sa tento parameter neuvedie, vráti hodnotu udelenej známky

Napríklad Naj(SJL9i, SJL8i)

Zobraziť návod
            
        


                
            ')]</value>
      <webElementGuid>35ecde3e-817f-4628-b6fc-b28d8ffa2ef8</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
