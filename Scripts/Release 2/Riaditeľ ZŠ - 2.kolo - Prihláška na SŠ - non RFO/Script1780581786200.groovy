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

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiSSNonRFO.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.selectOptionByLabel(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/2kolo/Select_PrihlaskyMojichZiakov2Kolo'), '2.kolo', false)

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/a_Export do xlsx_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-confirm govuk-button govuk-bu_4f0898'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zmenenaPracovnaSchopnostRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_S nadanm_specialneVVP_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mentlne postihnutie v kombinci s inm _972e2c'))

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Prida odbor mojej koly_ZStoSS'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Hada poda nzvu koly_hladat-podla-nazvu-skoly'), 
    'Stredná škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Hada poda nzvu koly_hladat-podla-naz_83928f'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_close_pridat-do-prihlasky govuk-butt_07df32'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-pridat-odbory-a-odist govuk_a933fa'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 4'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Uršula')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Zálesná')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_a04093'), 
    '855224/8826')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Email'), 
    'barcik@ditec.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421909655247')

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-eduidSkolyInput'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete ronk, ktor iak tuduje_select-r_31771c'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete triedu, ktor iak navtevuje v tv_d8d371'), 
    '9.A')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_5aad1f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_3'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Odhlsi_privatna-zona-content'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

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
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 8'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 9'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

def den = LocalDate.now().dayOfMonth

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyDen'), 
    den.toString())

def mesiac = LocalDate.now().monthValue

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyMesiac'), 
    mesiac.toString())

def rok = LocalDate.now().year

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyRok'), 
    rok.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    'poznámka školy')

'Krok 10'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_ziadostSkolskyRok'), 
    '2026 / 2027')

if (mesiac < 10) {
    mesiac = ('0' + mesiac.toString())
}

if (den < 10) {
    den = ('0' + den.toString())
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_ziadostDatumPodania'), 
    (((den + '.') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka koly_poznamkaSkoly'), 
    'poznámka školy')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kolo prijmacieho konania_koloPrijimacie_73f495'), 
    '2. kolo', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMenoSuhrn'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezviskoSuhrn'), 
    priezvisko.toString())

//rodne priezvisko
//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezviskoSuhrn'),'Horváth')
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCisloSuhrn'), 
    rc.toString())

Helper helper = new Helper()

def datumNarodenia = helper.rcToDatumNarodenia(rc.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodeniaSuhrn'), 
    datumNarodenia.toString())

String pohlavie = 'ženské'

if (helper.isMuz(rc.toString())) {
    pohlavie = 'mužské'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavieSuhrn'), 
    pohlavie)

//mesto
//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodeniaSuhrn'),'Nitra')
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnostSuhrn'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnostSuhrn'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazykSuhrn'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_b334cc'), 
    'Kráľa Mateja 654/23, 01514, Smižany, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zmenen pracovn schopnos_dpZmenenaPracov_8af9b9'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Upraven podmienky prijmacieho konania (_1d9ac6'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Mentlne postihnutie_dpMentalnePostihnutie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    '910021624')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Stredná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    '2285H00')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_3'), 
    'zlievač -3 ročné')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_4'), 
    'Netalentový')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_5'), 
    '1. termín')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_6'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_7'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_8'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Uršula')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Zálesná')

//rodne priezvisko
//WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupcaRodnePriezvisko'), 
//    'Zálesná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '855224/8826')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '24.02.1985')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Kráľa Mateja 654/23, 01514, Smižany, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421909655247')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_zastupcaNeznamy'), 
    'Druhý zákonný zástupca nie je známy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prchod iaka_prichodZiakaSuhrnZiadost'), 
    'Zo ZŠ na Slovensku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_EDUID zkladnej koly_eduidZSSuhrnZiadost'), 
    '910021625')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nzov zkladnej koly_nazovZSSuhrnZiadost'), 
    'Základná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Ronk_rocnikSuhrnZiadost'), 
    '9.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Trieda_triedaSuhrnZiadost'), 
    '9.A')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rok kolskej dochdzky_rokSkolskejDochadz_7fd364'), 
    '9')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Vyuovac jazyk v zkladnej kole_vyucovaci_4145b7'), 
    'Slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_9'), 
    'veľmi dobré')


WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Sae_panel-content'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'), 
    'Neboli nahrané žiadne prílohy.')

'Odoslanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

'Filtrovanie výsledkov'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/button_(nepovinn)_btn-zoradit-podla-predvolene'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/input_Poda priezviska (abecedne - vzostupne_a91dc1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/button_Sp_btn-zoradit govuk-button govuk-bu_4b9abc'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/div_Neskontrolovan_grey-label datum-label'), 
    (((den + '.') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/div_(nepovinn)_name-label'), 
    (priezvisko.toString() + ' ') + meno.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Prihlky naich iakov  ePrihlky/button_Akcia_govuk-button govuk-button--sec_dea96d'))

'Tlačidlo Zrušiť'
WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/RFO/Page_Podrobnosti prihlky  ePrihlky/button_Odosla na stredn koly_btn-zrusit-prihlasku'), 
    0)

prihlasovanie.odhlasPouzivatela()

