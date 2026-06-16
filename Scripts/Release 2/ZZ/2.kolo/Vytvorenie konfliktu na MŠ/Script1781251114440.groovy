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

Prihlasovanie prihlasovanie = new Prihlasovanie()

Subor subor = new Subor()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/detiMSNonRFO.txt')

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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_prihlaska1.pdf  Potvrdenie o zdravotne_b6beda'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e_1'))

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593020', 'hvisbbHiKeCSox23I94xOA==', GlobalVariable.F2A, '910021626')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/button_keyboard_arrow_down_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

//WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaRiad/bezRFO/PridatPapierovyKonfilktButton'))
WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'Slovenská re')

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

'3 krok'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

'4 krok'
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

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_c9f647'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-ziadost govuk-butto_d925c5'))

WebUI.waitForJQueryLoad(60, FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaMS/PapierovaPrihlaska/Page_Prihlky a rozhodnutia  ePrihlky/input_Povinn prihlky na prijatie, ale kvli _9491e4'), 
    (meno + ' ') + priezvisko)

subor.zapisUdajeNaPrenos(meno, priezvisko)