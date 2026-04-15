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

'Vytvorenie prihlášky ZZ'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Moje prihlky  ePrihlky/a_Prida existujcu prihlku_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Moje prihlky  ePrihlky/input_Prihlku mete poda od 1. oktbra do 31._8142a6'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Moje prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button'))

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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/chb_2ZZ'))

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete kolu, ktor iak navtevuje_skola-input'), 
    '910021625')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete posledn ukonen ronk zkladnej k_6a0368'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete triedu, ktor iak navtevuje v tv_d8d371'), 
    '9.A')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_5aad1f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/input_vyucovaciJazyk'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Odhlsi_privatna-zona-content'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-1-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 6
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-2-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 7
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-3-1'), 
    '29', true)

//vymazanie zvyšných známok okrem správania pre ročník 8
for (int i = 0; i < 18; i++) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_btn-odstranit govuk-button gov_f77c70'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-4-1'), 
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

'Krok 9'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.waitForJQueryLoad(20)

WebUI.verifyElementVisible(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'))

def identifikator = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_prihlaskaSkolskyRok'), 
    '2026/2027')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kolo prijmacieho konania_koloPrijimacie_73f495'), 
    '1. kolo', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCislo'), 
    rc.toString())

Helper helper = new Helper()

def datumNarodenia = helper.rcToDatumNarodenia(rc.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodenia'), 
    datumNarodenia.toString())

String pohlavie = 'žena'

if (helper.isMuz(rc.toString())) {
    pohlavie = 'muž'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavie'), 
    pohlavie.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodenia'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnost'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnost'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazyk'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_3d75f1'), 
    'Juríčková 896/2, 03657, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zmenen pracovn schopnos_dpZmenenaPracov_8af9b9'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_pecilne vchovno-vzdelvacie potreby_dpSVVP'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Mentlne postihnutie_dpMentalnePostihnutie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    '910021624')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Stredná škola pre AT', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    '2940M04')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_3'), 
    'potravinárstvo-kvasná technológia - 4 ročné')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_4'), 
    'Netalentový')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_5'), 
    '1. termín')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_6'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_7'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_8'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Tomáš')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Lukáč')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '561019/0003')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    'katalontest987@gmail.com')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Ražná 58/232, 01508, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '19.10.1956')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_zastupcaNeznamy'), 
    'Druhý zákonný zástupca nie je známy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prchod iaka_prichodZiakaSuhrn'), 
    'Zo ZŠ na Slovensku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_EDUID zkladnej koly_eduidZSSuhrn'), 
    '910021625')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nzov zkladnej koly_nazovZSSuhrn'), 
    'Základná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Ronk_rocnikSuhrn'), 
    '9.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Trieda_triedaSuhrn'), 
    '9.A')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rok kolskej dochdzky_rokSkolskejDochadzkySuhrn'), 
    '9')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Vyuovac jazyk v zkladnej kole_vyucovaci_4f3991'), 
    'Slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_9'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_10'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_11'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_12'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Sa_sutaz-item sutaz-nazov'), 
    'Klokaniáda')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Klokanida_sutaz-item sutaz-umiestnenie'), 
    '3. miesto - Okresná úroveň')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Klokanida_sutaz-item sutaz-druh'), 
    'Vedomostné vrátane predmetových')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Vedomostn vrtane predmetovch_sutaz-ite_ce6f60'), 
    'Školský rok: 2024/2025')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prlohy_prilohyContainer'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Dokument (1).pdf  Vysvedenie z 9. ronk_f25a28'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

'Odoslanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/button_odoslatPrihlasku'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/h1_Vytvorenie elektronickej prihlky_govuk-h_7c6a0b'), 
    'Prihláška bola úspešne odoslaná!')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Chcem posla sptn vzbu_btn-moje-prihl_0c72f2'))

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky a rozhodnutia  ePrihlky/span'))

'Kontrola na ZŠ'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/button_Akcia_govuk-button govuk-button--sec_dea96d'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/button_Exportova PDF_btn-oznacit-ako-skontrolovana'))

WebUI.waitForJQueryLoad(60)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'), 
    identifikator.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge'), 
    'Pripravená')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav kontroly_stavKontroly badge red'), 
    'Skontrolovaná')

WebUI.verifyElementVisible(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/button_OdoslatNaSS'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/button_OdoslatNaSS'))

WebUI.waitForJQueryLoad(60)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_infobannerOdoslanie'), 
    'Prihláška bola podaná priamo na strednú školu. Prihlášku pridal do systému riaditeľ školy Jalmová 266/19, 06534, Prešov')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge - Copy'), 
    'Podaná')

WebUI.verifyElementNotVisible(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/button_OdoslatNaSS'), 
    FailureHandling.STOP_ON_FAILURE)

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Prihlky a rozhodnutia  ePrihlky/div_(nepovinn)_name-label'), 
    (priezvisko.toString() + ' ') + meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Prihlky a rozhodnutia  ePrihlky/div_Neskontrolovan_badge'), 
    'Elektronicky')

'detail prihlášky SŠ'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator_1'), 
    identifikator.toString( // + '.01')
        ))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_kolsk rok_prihlaskaSkolskyRok'), 
    '2026/2027')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Spsob podania_prihlaskaSposobPodania'), 
    'Elektronicky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Kolo prijmacieho konania_koloPrijimacie_73f495'), 
    '1. kolo', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_stavPrihlasky badge_1'), 
    'V spracovaní')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Stav kontroly_stavKontroly badge red'), 
    'Neskontrolovaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_3d75f1'), 
    'Juríčková 896/2, 03657, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Zmenen pracovn schopnos_dpZmenenaPracov_8af9b9'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_pecilne vchovno-vzdelvacie potreby_dpSVVP'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Mentlne postihnutie_dpMentalnePostihnutie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div'), 
    '910021624')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_1'), 
    'Stredná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_2'), 
    '2940M04')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_3'), 
    'potravinárstvo - kvasná technológia - 4 ročné')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_4'), 
    'Netalentový')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_5'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Tomáš')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Lukáč')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Rodn slo_zakonnyZastupca1RodneCislo'), 
    '561019/0003')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '19.10.1956')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Prchod iaka_prichodZiakaSuhrn'), 
    'Zo ZŠ na Slovensku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_EDUID zkladnej koly_eduidZSSuhrn'), 
    '910021625')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Nzov zkladnej koly_nazovZSSuhrn'), 
    'Základná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Ronk_rocnikSuhrn'), 
    '9.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Trieda_triedaSuhrn'), 
    '9.A')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Rok kolskej dochdzky_rokSkolskejDochadzkySuhrn'), 
    '9')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Vyuovac jazyk v zkladnej kole_vyucovaci_4f3991'), 
    'Slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Sprvanie_red-span'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Sprvanie_red-span_1'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Sprvanie_red-span_2'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/div_Sprvanie_red-span_3'), 
    'veľmi dobré')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/span_Sa_sutaz-item sutaz-nazov'), 
    'Klokaniáda')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/a'), 
    0)

WebUI.verifyElementVisible(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/detailPrihlaskySS/Page_Podrobnosti prihlky  ePrihlky/button_vedomostn vrtane predmetovch_btn-vyz_863f82'))

prihlasovanie.odhlasPouzivatela()

