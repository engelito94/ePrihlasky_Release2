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
import portal.Prihlasovanie as Prihlasovanie
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021626')

'Profil školy\r\n'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Vyberte monos, ktor najpresnejie vyst_2a0e27'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Bez telocvine_radioGroup-MS_VnuV_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Bez vonkajieho ihriska_radioGroup-MS__be487e'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_68b493'), 
    'testovací popis 1')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Údaje profilu školy boli úspešne uložené')

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

'nájsť školu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_99fc84'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_cbc5a9'), 
    'Materská škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_57678e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/a_slovensk_govuk-link link-button viac-info_8d118b'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Nie s dostupn dta_zobrazit-profil-sk_4af58a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/div_Zavrie_title'), 
    'Materská škola pre AT, Balková 8')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/div_Wupertalsk 10, 041 01 Koice-Sdlisko KVP_9fdc95'), 
    'testovací popis 1')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span'), 
    'Škola neprístupná pre osoby so zníženou mobilitou')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_1'), 
    'Malá telocvičňa do 100 m2')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_2'), 
    'Základné vonkajšie ihrisko')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Zkladn vonkajie ihrisko_btn-zavriet _3c66e8'))

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021626')

'Profil školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Prihlky a rozhodnutia  ePrihlky/a_keyboard_arrow_down_govuk-header__link li_a691a3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Prstupn poschodia, toalety a jedle_ra_8cdb32'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Vek telocvia nad 400 m2_radioGroup-MS_baef3c'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/input_Multifunkn vonkajie ihrisko_radioGrou_7146f4'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_68b493'), 
    'testovací popis 2')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Údaje profilu školy boli úspešne uložené')

'odhlásenie'
prihlasovanie.odhlasPouzivatela()

'nájsť školu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_99fc84'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_cbc5a9'), 
    'Materská škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_57678e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Nie s dostupn dta_zobrazit-profil-sk_4af58a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/div_Wupertalsk 10, 041 01 Koice-Sdlisko KVP_9fdc95_1'), 
    'testovací popis 2')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_4'), 
    'Úplná debarierizácia všetkých priestorov a vonkajšieho areálu')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_5'), 
    'Viac telocviční')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/span_6'), 
    'Viaceré multifunkčné vonkajšie ihriská')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/MS/Page_Njs kolu  ePrihlky/button_Zkladn vonkajie ihrisko_btn-zavriet _3c66e8'))

