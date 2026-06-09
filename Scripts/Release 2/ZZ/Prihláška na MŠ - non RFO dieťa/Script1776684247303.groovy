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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Moje prihlky  ePrihlky/a_Prida existujcu prihlku_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Moje prihlky  ePrihlky/input_(nepovinn)_modalVytvoritPrihlaskuRadi_277a4d'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Laura Kredenc (14.3.2009)_radioGroup-_3a58ba'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Pridajte diea alebo osobu vo vaej starost_3d4c5c'))

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
    'Slovenská re')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'bôrka')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'), 
    'matun')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '123')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '321')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '75369')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_39a9fe'))

'1.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'2.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Poldenn vchovu a vzdelvanie_msCeloden_9f7220'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDSVVPRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__DPDDietaSNadanimRadio_option_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrijatiaDen'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrija_c62524'), 
    '9')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-pozadovanyDatumPrijatiaRok'), 
    '2026')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Odhlsi_privatna-zona-content'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'Nepapá kefíry')

'3.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Vybra koly_nav-item-najst-skolu-MS'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Hada poda mojej adresy_hladat-podla-r_99fc84'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_nazov-skol_cbc5a9'), 
    'Materská škola pre AT')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Nzov koly alebo jej adresa_nazov-sko_57678e'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Balkov 988, 36578, Bansk Bystrica_pr_e3437f'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'4.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421951362478')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio_option_1'))

'5.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prilote vetky potrebn prlohy_govuk-acco_a7b82c'))

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_alebo ho sem potiahnite (max. 10 MB, vo f_05689b'), 
    priloha)

WebUI.delay(1)

'6.krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

String identifikator = WebUI.getText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Identifiktor prihlky_prihlaskaIdentifikator'), 
    identifikator)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_prihlaskaSkolskyRok'), 
    '2026/2027')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum podania_prihlaskaDatumPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Spsob podania_prihlaskaSposobPodania'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMeno'), 
    meno.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    priezvisko.toString())

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCislo'), 
    rc.toString())

Helper help = new Helper()

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodenia'), 
    help.rcToDatumNarodenia(rc.toString()))

String pohlavie = 'žena'

if (help.isMuz(rc.toString())) {
    pohlavie = 'muž'
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavie'), 
    pohlavie)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodenia'), 
    'Slovensko')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnost'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnost'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazyk'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In materinsk jazyk_dietaInyMaterinskyJazyk'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_3d75f1'), 
    'Matunákova 123/321, 75369, Bôrka, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa miesta, kde sa diea obvykle zdri_a56ee1'), 
    'Matunákova 123/321, 75369, Bôrka, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_iadam o prijatie dieaa na_dpDietataMsCe_f5d5dc'), 
    'Celodennú výchovu a vzdelávanie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zdravotn znevhodnenie dieaa_dpDietataSV_1cfce1'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Diea s nadanm_dpDietaSNadanim'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poadovan dtum prijatia dieaa do matersk_673c9e'), 
    '01.09.2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poznmka_dpDietataPoznamka multiline-label'), 
    'Nepapá kefíry')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'), 
    'Materská škola pre AT')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    'Balková 98/8, 36578 Banská Bystrica')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupcaMeno'), 
    'Tomáš')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Lukáč')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_zakonnyZastupcaRodnePriezvisko'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '561019/0003')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '19.10.1956')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Ražná 58/232, 01508, Košariská, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    'katalontest987@gmail.com')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupcaTelefon'), 
    '+421951362478')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_prihlaska1.pdf  Potvrdenie o zdravotne_b6beda'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e_1'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/h1_Vytvorenie elektronickej prihlky_govuk-h_7c6a0b'), 
    'Prihláška bola úspešne odoslaná!')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021626')

'Kontrola na MŠ'
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno.toString() + ' ') + priezvisko.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Page_Prihlky naich iakov  ePrihlky/button_Vyhadvanie v prihlkach_fulltext-inpu_1e6782'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/ZobrazitPrihlaskuButton'))

WebUI.waitForJQueryLoad(100)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/IDprihlasky'), identifikator)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/StavPrihlasky'), 'Podaná')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/menoPrihlasky'), meno)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZS/priezviskoPrihlasky'), priezvisko)

subor.zapisUdajeNaPrenos(meno, priezvisko)

