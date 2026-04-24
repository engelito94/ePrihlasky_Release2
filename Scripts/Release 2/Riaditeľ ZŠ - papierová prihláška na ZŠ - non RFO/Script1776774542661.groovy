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

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/button_keyboard_arrow_down_btn-vytvorit-prihlasku'))

'Pridat dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'Slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'Predm')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(ak existuje  potrebn pre doruovanie _6f78ce'), 
    'Egrešs')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '12')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '01589')

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Predprimrne vzdelvanie_povinnePredprima_f31d60'), 
    '-')

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Etick_zsDPDVychovaRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Rmskokatolcka_zsDPDVychovaMoznostiRad_a6d2cb'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDStravovanieRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDSkolskyKlubRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDSVVPRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDDietaSNadanimRadio_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'ŠVVP')

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 4'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Jozef')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Mrkva')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_a04093'), 
    '660226/9146')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421963258471')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio_option_1'), 
    0)

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

def den = LocalDate.now().dayOfMonth

def mesiac = LocalDate.now().monthValue

def rok = LocalDate.now().year

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyDen'), 
    den.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyMesiac'), 
    mesiac.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumPodaniaPrihlaskyRok'), 
    rok.toString())

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    '(-_-)')

'Krok 7'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_ziadostIdentifikator'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_ziadostSkolskyRok'), 
    '2026 / 2027')

if (mesiac < 10) {
    mesiac = ('0' + mesiac.toString())
}

if (den < 10) {
    den = ('0' + den.toString())
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_ziadostDatumPodania'), 
    (((den + '.') + mesiac) + '.') + rok)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Spsob podania_ziadostSposobPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka koly_poznamkaSkoly'), 
    '(-_-)')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMenoSuhrn'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezviskoSuhrn'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezviskoSuhrn'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCisloSuhrn'), 
    rc.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodeniaSuhrn'), 
    help.rcToDatumNarodenia(rc.toString()))

String pohlavie = 'ženské'

if (help.isMuz(rc.toString())) {
    pohlavie = 'mužské'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavieSuhrn'), 
    pohlavie)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodeniaSuhrn'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnostSuhrn'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnostSuhrn'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazykSuhrn'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In materinsk jazyk_dietaInyMaterinskyJa_0014a5'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_b334cc'), 
    'Egrešská 12/1, 01589, Predmier, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa miesta, kde sa diea obvykle zdri_2f5e08'), 
    'Egrešská 12/1, 01589, Predmier, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poadovan vchova_dpDietataPozadovanaVychova'), 
    'Náboženská - Evanjelická')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zujem o stravovanie v kolskej jedlni_dp_ad0c47'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zujem o kolsk klub det_dpDietataDruzina'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zdravotn znevhodnenie dieaa_dpDietataSV_1cfce1'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka_dpDietataPoznamka'), 
    'ŠVVP')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    'Základná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Jalmová 266/19, 06534 Prešov')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    'slovenský', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Jozef')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Mrkva')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupcaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '660226/9146')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slo elektronickej schrnky_cisloElektron_dd0e2e'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '26.02.1966')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Egrešská 12/1, 01589, Predmier, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421963258471')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_zastupcaNeznamy'), 
    'Druhý zákonný zástupca nie je známy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'), 
    'Neboli nahrané žiadne prílohy.')

'Odoslanie prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/span_check_circle_panel-text'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/input_Povinn prihlky na prijatie, ale kvli _9491e4'), 
    (meno + '') + priezvisko)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/button_Povinn prihlky na prijatie, ale kvli_21db34'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/div_Egresk 121,01589 Predmier_badge'), 
    'Papierovo')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/div_Neskontrolovan_data-prihlaska-stav badge green'), 
    'Podaná')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Prihlky a rozhodnutia  ePrihlky/button_Detail_govuk-button govuk-button--se_40abef'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Podrobnosti prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Podrobnosti prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Podrobnosti prihlky  ePrihlky/div_Stav prihlky_skola-status-badge green'), 
    'Podaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Papier/Page_Podrobnosti prihlky  ePrihlky/div'), 
    'slovenský', FailureHandling.OPTIONAL)

