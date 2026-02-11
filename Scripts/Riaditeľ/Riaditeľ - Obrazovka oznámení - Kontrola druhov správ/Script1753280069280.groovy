import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

Portal portal = new Portal()

portal.prihlasUcet('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, true)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Prihlky a rozhodnutia  ePrihlky/a_Sprva pouvateov'))

'Správa používateľov'
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/karta_SPravaPouzivatelov'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/button_Odstrni'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/button_PotvrditZmazanie'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/div_Pouvatea ste spene odstrnili'), 
    'Používateľa ste úspešne odstránili.')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/button_Prida pouvatea'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/button_Prida'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/span_Pouvatea ste spene pridali'), 
    'Používateľa ste úspešne pridali.')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/span_Zuzana Majdov'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Sprva pouvateov  ePrihlky/span_KZ'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Mj profil  ePrihlky/a_Oznmenia'))

WebUI.waitForJQueryLoad(30)

'Obrazovka oznámení'
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/input_(nepovinn)_input-datum'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/li_Dnes'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Vetky                                  _330ccb'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Odstrnenie role pouvatea'))

WebUI.waitForJQueryLoad(30)

'Odstránenie role'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_spene ste odstrnili pouvateovi jeho rolu'), 
    'Úspešne ste odstránili používateľovi jeho rolu')

WebUI.verifyTextPresent('Úspešne ste odstránili používateľovi jeho rolu.', false)

WebUI.verifyTextPresent('Používateľ: Peter Kladivko', false)

WebUI.verifyTextPresent('Rola: Spracovateľ', false)

//String text = WebUI.getText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_spene ste odstrnili pouvateovi jeho rol_93c6e9'))
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Nepretan'), 'Neprečítaná')

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Odstrnenie role pouvatea'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Priradenie role pouvatea'))

'Priradenie role'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_spene ste priradili pouvateovi jeho rolu'), 
    'Úspešne ste priradili používateľovi jeho rolu')

WebUI.verifyTextPresent('Úspešne ste priradili používateľovi jeho rolu.', false)

WebUI.verifyTextPresent('Používateľ: Peter Kladivko', false)

WebUI.verifyTextPresent('Rola: Spracovateľ', false)

//text = WebUI.getText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_spene ste odstrnili pouvateovi jeho rol_93c6e9'))
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Nepretan'), 'Neprečítaná')

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Priradenie role pouvatea'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Vygenerovanie rozhodnut'))

'Vygenerovanie rozhodnutia'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Vsledok vygenerovania rozhodnut'), 
    'Výsledok vygenerovania rozhodnutí')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Nepretan (1)'), 
    'Neprečítaná')

WebUI.verifyTextPresent('Dobrý deň,', false)

WebUI.verifyTextPresent('oznamujeme Vám, že generovanie rozhodnutí pre Vašu školu Materská škola bolo ukončené.', false)

WebUI.verifyTextPresent('Bolo vygenerovaných 1 rozhodnutí o prijatí a 0 rozhodnutí o neprijatí.', false)

WebUI.verifyTextPresent('S pozdravom', false)

WebUI.verifyTextPresent('Tím elektronických prihlášok MŠVVaM SR', false)

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Vygenerovanie rozhodnut'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Doplnenie prloh'))

'Doplnenie príloh'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Poadovan prlohy boli doplnen'), 
    'Požadované prílohy boli doplnené')

WebUI.verifyTextPresent('Zákonný zástupca pridal požadované prílohy k prihláške', false)

WebUI.verifyTextPresent('Poznámka: AT - priložená príloha', false, FailureHandling.OPTIONAL)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Prejs na prihlku'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan (1)'), 'Neprečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Doplnenie prloh'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_iados o doplnenie informci'))

'Vyžiadanie prílohy'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Vyiadali ste od Zkonnho zstupcu doplujc_4364ec'), 
    'Vyžiadali ste od Zákonného zástupcu doplňujúce informácie')

WebUI.verifyTextPresent('Vážený/á pán/pani Karolína Lietajúca,', false, FailureHandling.OPTIONAL)

WebUI.verifyTextPresent('škola Materská škola požaduje doplnenie Vami podanej prihlášky.', false)

WebUI.verifyTextPresent('Dôvod:', false)

WebUI.verifyTextPresent('AT test - vyžiadanie prílohy', false, FailureHandling.OPTIONAL)

WebUI.verifyTextPresent('Pre doplnenie požadovaných príloh sa prihláste do špecializovaného portálu.', false)

WebUI.verifyTextPresent('S pozdravom', false)

WebUI.verifyTextPresent('Tím elektronických prihlášok MŠVVaM SR', false)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Prejs na prihlku'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan (1)'), 'Neprečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_iados o doplnenie informci'))

/**
 * 
 * 
 * WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Pridanie role pouvatea'))

WebUI.delay(10)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Bola Vm priraden rola na zklade ktorej _2fc4a9'), 
    'Bola Vám priradená rola na základe ktorej môžete spracúvať prihlášky')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Bola Vm priraden rola na zklade ktorej _e0f97d'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Nepretan'), 'Neprečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Pridanie role pouvatea'))
**
**
***/
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Vetky                                  _330ccb_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Nepretan'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Oznai vetky ako pretan'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/span_Zadanm vyhadvacm kritrim nevyhovuje ia_2c872a'), 
    'Zadaným vyhľadávacím kritériám nevyhovuje žiadne oznámenie.')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/span_Nepretan'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Nepretan'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Pretan'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Vetky                                  _330ccb'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Odstrnenie role pouvatea'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan'), 'Prečítaná')

//WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/span_Odstrnenie role pouvatea'))
WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Odstrnenie role pouvatea'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Priradenie role pouvatea'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan'), 'Prečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Priradenie role pouvatea'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Vygenerovanie rozhodnut'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan'), 'Prečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Vygenerovanie rozhodnut'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Doplnenie prloh'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan'), 'Prečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_Doplnenie prloh'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/label_iados o doplnenie informci'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_Pretan'), 'Prečítaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/div_KZ'))

WebUI.click(findTestObject('Object Repository/Zak_test/Oznamenia/Page_Oznmenia  ePrihlky/a_Odhlsi'))

