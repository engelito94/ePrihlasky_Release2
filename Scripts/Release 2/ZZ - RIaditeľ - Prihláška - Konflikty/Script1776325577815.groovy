import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import java.time.LocalDate as LocalDate
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

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
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
import portal.Helper as Helper
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

Subor subor = new Subor()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiSSNonRFO.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

prihlasovanie.prihlasPouzivatela('ljxikynq7v@dollicons.com', 'w1oXMoeykcdLiib/wAKM5A==', false, GlobalVariable.F2A)

'Vytvoriť prihlášku'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/a_Prida existujcu prihlku_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/input_Prihlku mete poda od 1. oktbra do 31._8142a6'))

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button'))

'Vytvorenie dieťaťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Laura Kredenc (14.3.2009)_radioGroup-_3a58ba'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Pridajte diea alebo osobu vo vaej starost_3d4c5c'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'),
	rc.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'),
	meno.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'),
	priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-dalej govuk-button govuk-bu_a288b6'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'),
	'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'),
	'Sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'),
	'košari')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'),
	'juric')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'),
	'896')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'),
	'2')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'),
	'03657')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_39a9fe'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zmenenaPracovnaSchopnostRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_S nadanm_specialneVVP_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mentlne postihnutie v kombinci s inm _972e2c'))

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Stredná škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_9dda30'), 
    '11', true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Button_Skontrolovane'))

'Krok 4'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'),
	'+421962478632')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio_option_1'))

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_down_prichodZiakaRadio_10f409'))

WebUI.verifyElementChecked(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_down_prichodZiakaRadio_10f409'), 
    0)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete posledn ukonen ronk zkladnej k_a84d95'), 
    '9', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_f07e4f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Aktualizova a ods_idsk-footer-extended-_552556'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-1-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 6
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-2-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 7
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-3-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 8
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-4-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 9
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

'Krok 7'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-modalNazovSutazeText'),
	'klokaniáda')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalDruhSutazeSelect'),
	'1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalUrovenSutazeSelect'),
	'4', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_modalTypUmiestneniaRadio_option_2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_V ktorom sa iak zastnil sae_select-m_326ab7'),
	'2024/2025', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button govuk-b_106909'))

'Krok 8'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2'))

//vysvedcenie 6
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'), 
    priloha)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_1'))

//vysvedcenie 7
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_1'), 
    priloha)

WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_pril3'))

//vysvedcenie 8
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_2'), 
    priloha)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_2'))

//vysvedcenie 9
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_3'), 
    priloha)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_vedomostn vrtane predmetovch_material-_a57dce'))

//sutaze
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_4'), 
    priloha)

'Krok 9'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.waitForJQueryLoad(20)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Dokument (1).pdf  Vysvedenie z 9. ronk_f25a28'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

'Odoslanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/button_odoslatPrihlasku'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/h1_Vytvorenie elektronickej prihlky_govuk-h_7c6a0b'),
	'Prihláška bola úspešne odoslaná!')

prihlasovanie.odhlasPouzivatela()

//Vytvorenie konfliktnej papierovej prihlášky
prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/a_Export do xlsx_btn-vytvorit-prihlasku'))

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/PridatPapierovyKonfilktButton'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'smiža')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'), 
    'kráľ')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '654')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '23')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '01514')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zmenenaPracovnaSchopnostRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_S nadanm_specialneVVP_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mentlne postihnutie v kombinci s inm _972e2c'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'test ŠVVP poznámky')

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'))

WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_pridat-do-prihlasky govuk-butt_07df32_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-pridat-odbory-a-odist govuk_a933fa'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 4'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Uršula')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Zálesná')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_a04093'), 
    '855224/8826')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Email'), 
    'katalontest789@gmail.com')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421908452398')

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-eduidSkolyInput'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-eduidSkolyInput'), 
    '910021625')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete ronk, ktor iak tuduje_select-r_31771c'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete triedu, ktor iak navtevuje v tv_d8d371'), 
    '9.A')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_5aad1f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Odhlsi_privatna-zona-content'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-1-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 6
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87Riad'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-2-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 7
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87Riad'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-3-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 8
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87Riad'))

WebUI.selectOptionByValue(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-4-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 9
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

'Krok 7'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-modalNazovSutazeText'), 
    'Smižankovice')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalDruhSutazeSelect'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalDruhSportuSelect'), 
    '103', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalUrovenSutazeSelect'), 
    '6', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_(nepovinn)_govuk-label govuk-radios__label'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_V ktorom sa iak zastnil sae_select-m_326ab7'), 
    '2024/2025', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button govuk-b_106909'))

'Krok 8'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2'))

//vysvedcenie 6
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'), 
    priloha)

'Krok 9'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

def den = LocalDate.now().dayOfMonth

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyDen'), 
    den.toString())

def mesiac = LocalDate.now().monthValue

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyMesiac'), 
    mesiac.toString())

def rok = LocalDate.now().year

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyRok'), 
    rok.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    'Konflikt')

'Krok 10'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Pridanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/span_check_circle_panel-text'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/div_(nepovinn)_name-label'), 
    (priezvisko.toString() + ' ') + meno.toString())

prihlasovanie.odhlasPouzivatela()

