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

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiZSNonRFO.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

prihlasovanie.prihlasPouzivatela('ljxikynq7v@dollicons.com', 'w1oXMoeykcdLiib/wAKM5A==', false, GlobalVariable.F2A)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Moje prihlky  ePrihlky/a_Prida existujcu prihlku_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Moje prihlky  ePrihlky/input_Prihlku mete poda od 1. oktbra do 31._19903a'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button'))

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
    'abrahá')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '58')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '25987')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_39a9fe'))

'Krok 1'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zsDPDVychovaRadio_option_0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDStravovanieRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDSkolskyKlubRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDSVVPRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDDietaSNadanimRadio_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'Doplňujúce info o dieťati')

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.waitForJQueryLoad(100)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Matersk koly_nav-item-najst-skolu-ZS'))

WebUI.waitForJQueryLoad(100)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_df1d5a'))

WebUI.waitForJQueryLoad(100)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_b4d569'), 
    'Základná škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_d9d2c0'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Jalmov 26619, 06534, Preov_pridat-do_65e16b'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 4'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zastupca1Meno'), 
    'Tomáš')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zastupca1Priezvisko'), 
    'Lukáč')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zastupca1RodneCislo'), 
    '561019/0003')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zastupca1DatumNarodenia'), 
    '19.10.1956')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kontaktn e-mail_zastupca1Email'), 
    'katalontest987@gmail.com')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421963258741')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Meno'), 
    'Ada')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Priezvisko'), 
    'Lovelace')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_626081'), 
    '')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_626081'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_626081'), 
    '695213/0779')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Na uveden e-mailov adresu bude zkonnm_20a95f'), 
    'mail@mail.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_5fd01d'), 
    '+421987456321')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_agreementRadio_option_0'))

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 6'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

String identifikator = WebUI.getText(findTestObject('Zak_test/Release2/PrihlaskaZS/SuhrnnyPrehladID'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_prihlaskaSkolskyRok'), 
    '2026/2027')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_prihlaskaDatumPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Spsob podania_prihlaskaSposobPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCislo'), 
    rc.toString())

Helper help = new Helper()

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodenia'), 
    help.rcToDatumNarodenia(rc.toString()))

String pohlavie = 'žena'

if (help.isMuz(rc.toString())) {
    pohlavie = 'muž'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavie'), 
    pohlavie)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodenia'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnost'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnost'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazyk'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In materinsk jazyk_dietaInyMaterinskyJazyk'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_3d75f1'), 
    'Abrahám 58, 25987, Abrahám, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa miesta, kde sa diea obvykle zdri_a56ee1'), 
    'Abrahám 58, 25987, Abrahám, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poadovan vchova_dpDietataPozadovanaVychova'), 
    'Etická')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zujem o stravovanie v kolskej jedlni_dp_ad0c47'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zujem o kolsk klub det_dpDietataDruzina'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zdravotn znevhodnenie dieaa_dpDietataSV_1cfce1'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Diea s nadanm_dpDietaSNadanim'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka_dpDietataPoznamka multiline-label'), 
    'Doplňujúce info o dieťati')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    'Jalmová 266/19, 06534 Prešov')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nzov koly_nazov-skoly'), 
    'Základná škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    'Jalmová 266/19, 06534 Prešov')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Tomáš')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Lukáč')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupca2RodnePr_cac0a6'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupcaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '561019/0003')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '19.10.1956')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Ražná 58/232, 01508, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    'katalontest987@gmail.com')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421963258741')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Shlas s komunikciou vhradne so zkonnm z_d82d0d'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupca2Meno'), 
    'Ada')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupca2Priezvisko'), 
    'Lovelace')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupca2RodneCislo'), 
    '695213/0779')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupca2DatumNarodenia'), 
    '13.02.1969')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupca2Adre_f33ee1'), 
    'Ražná 58/232, 01508, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupca2Email'), 
    'mail@mail.sk')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupca2Telefon'), 
    '+421987456321')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_estne vyhlasujem, e s podanm prihlky sh_4a84e8'), 
    'Áno')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'), 
    'Neboli nahrané žiadne prílohy.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Neboli nahran iadne prlohy_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

'Odoslať prihlášku'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Chcem posla sptn vzbu_btn-moje-prihl_0c72f2'))

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021625')

'Kontrola na ZŠ'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/ZobrazitPrihlaskuButton'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/IDprihlasky'), identifikator)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/StavPrihlasky'), 'Podaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/menoPrihlasky'), meno)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/priezviskoPrihlasky'), priezvisko)

