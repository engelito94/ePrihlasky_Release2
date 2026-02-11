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
import internal.GlobalVariable
import portal.Prihlasovanie

import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, '910016010')

'Zmena profilu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_Prstupn przemie a toalety_radioGroup-_dcd5ca'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/label_Bez telocvine_govuk-label govuk-radio_7bf757'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/label_Zkladn vonkajie ihrisko_govuk-label g_31927a'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_(nepovinn)_radioGroup-ZS_DigR_option_2'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_no_radioGroup-ZS_WiFi_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_fc711e'), 
    'testovací popis')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

'Nájsť školu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/li_Matersk koly_nav-item-najst-skolu-ZS'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_df1d5a'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_b4d569'), 
    'Sobrance')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_d9d2c0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nie s dostupn dta_zobrazit-profil-sk_4af58a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Zavrie_title'), 
    'Základná škola, Komenského 12')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Zkladn kola, Komenskho 12_description'), 
    'Komenského 12, 073 01 Sobrance')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Komenskho 12, 073 01 Sobrance_hodnoty'), 
    'testovací popis')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_1'), 
    'Prístupné prízemie, toalety a jedáleň')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_2'), 
    'Malá telocvičňa do 100 m2')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_3'), 
    'Multifunkčné vonkajšie ihrisko')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_1'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_2'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_3'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_4'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_4'), 
    'Nie')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nie_btn-zavriet govuk-button govuk-b_0d2f24'))

prihlasovanie.prihlasRiaditela('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, '910016010')

'Profil školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Prihlky a rozhodnutia  ePrihlky/a_keyboard_arrow_down_govuk-header__link li_a691a3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_kola neprstupn pre osoby so znenou mo_db7689'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_Telocvia do 400 m2_radioGroup-ZS_VnuV_172c64'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_Bez vonkajieho ihriska_radioGroup-ZS__0cbda3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_(nepovinn)_radioGroup-ZS_DigR_option_3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/input_Vyberte monos_radioGroup-ZS_WiFi_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/textarea_Pridajte krtky popis vaej koly_tex_fc711e'), 
    'testovací popis 1')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/button_Popte pecilne vybavenie, vae spechy _caed26'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Údaje profilu školy boli úspešne uložené')

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

'Nájsť školu'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Domov_govuk-header__link'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/li_Matersk koly_nav-item-najst-skolu-ZS'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_df1d5a'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_b4d569'), 
    'Sobrance')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_d9d2c0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nie s dostupn dta_zobrazit-profil-sk_4af58a'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Zavrie_title'), 
    'Základná škola, Komenského 12')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Zkladn kola, Komenskho 12_description'), 
    'Komenského 12, 073 01 Sobrance')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/div_Komenskho 12, 073 01 Sobrance_hodnoty_1'), 
    'testovací popis 1')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_5'), 
    'Prístupné prízemie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_6'), 
    'Veľká telocvičňa nad 400 m2')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_7'), 
    'Základné vonkajšie ihrisko')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_1'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_2'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_5'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_rove poskytovanho digitlneho rozvoja i_6e7296_4'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/span_8'), 
    'Áno')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/ProfilSkoly/ZS/Page_Njs kolu  ePrihlky/button_Nie_btn-zavriet govuk-button govuk-b_0d2f24'))