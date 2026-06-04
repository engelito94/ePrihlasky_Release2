import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.time.LocalDate as LocalDate
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
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import portal.Helper as Helper
import org.openqa.selenium.Keys as Keys

Subor subor = new Subor()

Prihlasovanie prihlasovanie = new Prihlasovanie()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiSSNonRFO.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021624')

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_2kolo'), '2.kolo', false)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/a_Export do xlsx_btn-vytvorit-prihlasku'))

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_error_panel-text'), 
    'Rodné číslo dieťaťa sa nepodarilo overiť. Prosím, zadajte údaje manuálne')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'slovenská re')

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
    'barcik@ditec.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421908452398')

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-eduidSkolyInput'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-eduidSkolyInput'), 
    '910020024')

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

/**
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_1'))

//vysvedcenie 7
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_1'), 
    'C://Users//barcik//Downloads//Dokument (1).pdf')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_2'))

//vysvedcenie 8
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_2'), 
    'C://Users//barcik//Downloads//Dokument (1).pdf')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nahran_material-icons govuk-accordion-_97dfa2_3'))

//vysvedcenie 9
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_3'), 
    'C://Users//barcik//Downloads//Dokument (1).pdf')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Bez umiestnenia_material-icons govuk-a_eaf990'))

//sutaze
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b_4'), 
    'C://Users//barcik//Downloads//Dokument (1).pdf')
**/
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
    'poznámka k ostatným údajom')

'Krok 10'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_ziadostSkolskyRok'), 
    '2026 / 2027')

if (mesiac < 10) {
    mesiac = ('0' + mesiac.toString())
}

if (den < 10) {
    den = ('0' + den.toString())
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_ziadostDatumPodania'), 
    (((den + '.') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka koly_poznamkaSkoly'), 
    'poznámka k ostatným údajom')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kolo prijmacieho konania_koloPrijimacie_73f495'), 
    '1. kolo', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMenoSuhrn'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezviskoSuhrn'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCisloSuhrn'), 
    rc.toString())

Helper helper = new Helper()

def datumNarodenia = helper.rcToDatumNarodenia(rc.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodeniaSuhrn'), 
    datumNarodenia.toString())

String pohlavie = 'ženské'

if (helper.isMuz(rc.toString())) {
    pohlavie = 'mužské'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavieSuhrn'), 
    pohlavie)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodeniaSuhrn'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnostSuhrn'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnostSuhrn'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazykSuhrn'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_b334cc'), 
    'Kráľa Mateja 654/23, 01514, Smižany, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zmenen pracovn schopnos_dpZmenenaPracov_8af9b9'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Mentlne postihnutie_dpMentalnePostihnutie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka_dpPoznamka'), 
    'test ŠVVP poznámky')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Upraven podmienky prijmacieho konania (_1d9ac6'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    '910021624')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Stredná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    '2940M04')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_3'), 
    'potravinárstvo-kvasná technológia - 4 ročné')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_4'), 
    'Netalentový')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_5'), 
    '1. termín')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_6'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_7'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_8'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Uršula')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Zálesná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '855224/8826')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '24.02.1985')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Kráľa Mateja 654/23, 01514, Smižany, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    'barcik@ditec.sk')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421908452398')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prchod iaka_prichodZiakaSuhrnZiadost'), 
    'Zo ZŠ na Slovensku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_EDUID zkladnej koly_eduidZSSuhrnZiadost'), 
    '910020024')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nzov zkladnej koly_nazovZSSuhrnZiadost'), 
    'Základná škola s materskou školou, Soblahov 404')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Ronk_rocnikSuhrnZiadost'), 
    '9.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Trieda_triedaSuhrnZiadost'), 
    '9.A')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rok kolskej dochdzky_rokSkolskejDochadz_7fd364'), 
    '9')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Vyuovac jazyk v zkladnej kole_vyucovaci_4145b7'), 
    'Slovenský')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Vsledky vzdelvania na zkladnej kole_pan_df9b6e'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Sa_sutaz-item sutaz-nazov'), 
    'Smižankovice')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Smiankovice_sutaz-item sutaz-umiestnenie'), 
    'Bez umiestnenia - Školská úroveň')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Bez umiestnenia - kolsk rove_sutaz-ite_d696ed'), 
    'Športové - Cyklistika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_portov - Cyklistika_sutaz-item sutaz-s_da911b'), 
    'Školský rok: 2024/2025')

'Pridanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/span_check_circle_panel-text'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/div_(nepovinn)_name-label'), 
    (priezvisko.toString() + ' ') + meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Prihlky a rozhodnutia  ePrihlky/div_Neskontrolovan_badge'), 
    'Papierovo')

