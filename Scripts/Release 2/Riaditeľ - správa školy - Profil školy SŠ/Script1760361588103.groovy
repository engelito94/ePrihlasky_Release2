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

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910020859')

'Správa účtu - profil školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Vyberte monos, ktor najpresnejie vyst_02a195'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Vyberte monos, ktor najpresnejie vyst_e27a2c'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Vyberte monos, ktor najpresnejie vyst_db1d37'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_(nepovinn)_radioGroup-SS_DigR_option_0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Vyberte monos_radioGroup-SS_WiFi_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_96bed3'), 
    'testovací popis 1')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Údaje profilu školy boli úspešne uložené')

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

'Vyhľadanie školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.waitForJQueryLoad(120)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Gymnázium Metodova')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_Karirov poradca 1_zobrazit-profil-sk_0b8773'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/div_Zavrie_title'), 
    'Gymnázium Metodova')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/div_Nov 219, 04443 Budimr_hodnoty'), 
    'testovací popis 1')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_1'), 
    'Škola neprístupná pre osoby so zníženou mobilitou')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_2'), 
    'Bez telocvične')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_3'), 
    'Bez vonkajšieho ihriska')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/div'), 'Áno')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_1'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_2'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_3'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_4'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_no_btn-zavriet govuk-button govuk-bu_28c1b8'))

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910020859')

'Správa účtu - profil školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Prstupn poschodia, toalety a jedle_ra_1f68a1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Vek telocvia nad 400 m2_radioGroup-SS_a31607'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_Multifunkn vonkajie ihrisko_radioGrou_844adc'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_(nepovinn)_radioGroup-SS_DigR_option_4'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/input_no_radioGroup-SS_WiFi_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_96bed3'), 
    'testovací popis 2')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Údaje profilu školy boli úspešne uložené')

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

'Vyhľadanie školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Gymnázium Metodova')

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span'))

WebUI.waitForJQueryLoad(120)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_Karirov poradca 1_zobrazit-profil-sk_0b8773'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/div_Zavrie_title'), 
    'Gymnázium Metodova')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/div_Nov 219, 04443 Budimr_hodnoty_1'), 
    'testovací popis 2')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_4'), 
    'Úplná debarierizácia všetkých priestorov a vonkajšieho areálu')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_5'), 
    'Viac telocviční')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_6'), 
    'Viaceré multifunkčné vonkajšie ihriská, bežecká dráha')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_5'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_6'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_7'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_8'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/span_7'), 
    'Nie')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/Page_Njs kolu  ePrihlky/button_no_btn-zavriet govuk-button govuk-bu_28c1b8'))

