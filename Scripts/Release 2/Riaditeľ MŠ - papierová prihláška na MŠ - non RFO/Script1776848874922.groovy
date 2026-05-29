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
import portal.Helper as Helper
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import org.openqa.selenium.Keys as Keys

Mail mail = new Mail()

Helper help = new Helper()

Subor subor = new Subor()

def filePath = RunConfiguration.getProjectDir()

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiZSNonRFO.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021626')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/button_keyboard_arrow_down_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'Bašov')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(ak existuje  potrebn pre doruovanie _6f78ce'), 
    'Tupole')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '6287')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '9')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '60507')

'1 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'2 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_msCelodennaVychovaRadio_option_0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDSVVPRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDDietaSNadanimRadio_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrijatiaDen'), 
    '01')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrija_c62524'), 
    '09')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrijatiaRok'), 
    '2026')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'ŠVVP')

'3 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'4 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Svetlomíra')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Tajomná')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_a04093'), 
    '845124/7882')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421951753654')

'5 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prilote vetky potrebn prlohy_govuk-acco_a7b82c'))

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

def den = LocalDate.now().dayOfMonth

def mesiac = LocalDate.now().monthValue

def rok = LocalDate.now().year

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyDen'), 
    den.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyMesiac'), 
    mesiac.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyRok'), 
    rok.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    '(*-*)')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_ziadostIdentifikator'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_ziadostSkolskyRok'), 
    '2026 / 2027')

if (mesiac < 10) {
    mesiac = ('0' + mesiac.toString())
}

if (den < 10) {
    den = ('0' + den.toString())
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_ziadostDatumPodania'), 
    (((den + '.') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Spsob podania_ziadostSposobPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka koly_poznamkaSkoly'), 
    '(*-*)')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMenoSuhrn'), 
    meno)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezviskoSuhrn'), 
    priezvisko)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezviskoSuhrn'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCisloSuhrn'), 
    rc)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodeniaSuhrn'), 
    help.rcToDatumNarodenia(rc.toString()))

String pohlavie = 'ženské'

if (help.isMuz(rc.toString())) {
    pohlavie = 'mužské'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavieSuhrn'), 
    pohlavie)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodeniaSuhrn'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnostSuhrn'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnostSuhrn'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazykSuhrn'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In materinsk jazyk_dietaInyMaterinskyJa_0014a5'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_b334cc'), 
    'Tupolevova 6287/9, 60507, Bašovce, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa miesta, kde sa diea obvykle zdri_2f5e08'), 
    'Tupolevova 6287/9, 60507, Bašovce, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_iadam o prijatie dieaa na_dpDietataMsCe_f5d5dc'), 
    'Poldennú výchovu a vzdelávanie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zdravotn znevhodnenie dieaa_dpDietataSV_1cfce1'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Diea s nadanm_dpDietataSVVPotrebySNadanim'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poadovan dtum prijatia dieaa do matersk_673c9e'), 
    '01.09.2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka_dpDietataPoznamka'), 
    'ŠVVP')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    'Materská škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Balková 98/8, 36578 Banská Bystrica')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Svetlomíra')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Tajomná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupcaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '845124/7882')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slo elektronickej schrnky_cisloElektron_dd0e2e'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '24.01.1984')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Tupolevova 6287/9, 60507, Bašovce, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421951753654')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_zastupcaNeznamy'), 
    'Druhý zákonný zástupca nie je známy.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

WebUI.waitForJQueryLoad(60, FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/span_check_circle_panel-text'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/input_Povinn prihlky na prijatie, ale kvli _9491e4'), 
    (meno + ' ') + priezvisko)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/button_Povinn prihlky na prijatie, ale kvli_21db34'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko)

