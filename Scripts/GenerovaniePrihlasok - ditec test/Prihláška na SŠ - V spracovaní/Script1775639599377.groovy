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
import portal.Prihlasovanie as Prihlasovanie
import portal.Subor as Subor
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

Subor subor = new Subor()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

def udaje = subor.dajDietaRiadSS(filePath + '/Data Files/ZZdetiDitec.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def rc = udaje.rc

//def rodnePriezvisko = udaje.cislo
/*
 * Prihlasovanie riaditeľa
prihlasovanie.prihlasRiaditela("930074646", "MNDMEHzxmeo0UUgzO9L2AQ==", false, "910001965")

prihlasovanie.odhlasPouzivatela()
*/
/*
 * Prihlasovanie ZZ
 */
//prihlasovanie.prihlasPouzivatela('barcik@ditec.sk', 'w1oXMoeykcdLiib/wAKM5A==', false)
prihlasovanie.prihlasPouzivatela('wxim4ueaws@deltajohnsons.com', 'w1oXMoeykcdLiib/wAKM5A==', false)


WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Moje prihlky  ePrihlky/a_Prida existujcu prihlku_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Moje prihlky  ePrihlky/input_Prihlku mete poda od 1. aprla do 30. _a37059'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Moje prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Jano lota (02.1.2010)_radioGroup-deti_c3b2ae'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Pridajte diea alebo osobu vo vaej starost_3d4c5c'))

/*
 * Vytváranie dieťaťa
 */
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio_option_0'))

//RČ
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-ro_fcd9b1'), 
    rc)

//Meno
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

//Priezvisko
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

//Rodné priezvisko
//WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-rodnePriezvisko'), rodnePriezvisko)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-dalej govuk-button govuk-bu_a288b6'))

//Adresa - štát
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In adresa trvalho pobytu_adresaTP'))

//Mesto
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'Bratislava-L')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In adresa trvalho pobytu_adresaTP_1'))

//Ulica
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(ak existuje  potrebn pre doruovanie _6f78ce'), 
    'Karlovar')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_Karlovarsk_govuk-label'))

//Súpisné čislo
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '25')

//Orientačné číslo
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '589')

//PSČ
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '45369')

//Uloženie dieťaťa
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp_btn-dalej govuk-button govuk-butt_39a9fe'))

/*
 * Vytvorenie prvého kroku
 */
'1.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'2.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

//WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zmenenaPracovnaSchopnostRa_255e07'))
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zmenenaPracovnaSchopnostRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_S nadanm_specialneVVP_option_1'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mentlne postihnutie v kombinci s inm _972e2c'))

'3.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

/*
'Vyhľadanie SŠ'
*/
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Zkladn koly_nav-item-najst-skolu-SS'))
/**
// Automat 1 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Automat 1')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/chb_2'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/VymazFilter'))

// Automat 2 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Automat 2')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/chb_2 - Copy'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/VymazFilter'))

// Automat 3 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Automat 3')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/chb_2 - Copy (1)'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/VymazFilter'))

/**
// Automat 5 - netalent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Automat 5')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/chb_2 - Copy (2) - Copy'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/VymazFilter'))
**//**
// Automat 4 - talent 1
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'Automat 4')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Nzov odboru_placeholder'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/chb_2 - Copy (2)'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_keyboard_arrow_up_govuk-button govuk_25831c'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/VymazFilter'))
**/
WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/FilterSKoly/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'),
	'Stredná Škola Bratislava Lesná')
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))
//WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef_1'))
//WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef_2'))
//WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef_3'))

'Nastavenie termínov pre odbory'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_9dda30'), 
    '11', true)

//WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_108023'), '21', true) //21

//WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_09a6c5'), '11', true)
//WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_c48eec'),  '21', true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'4.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/KontrolaZS/Button_Skontrolovane'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421999888777')

/**
WebUI.setText(findTestObject('Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input - Copy'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In korepondenn adresa_zastupca1InaAdresa'))

WebUI.setText(findTestObject('Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1 - Copy'), 
    'Slati')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Ulica_optional-text'))

WebUI.setText(findTestObject('Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(ak existuje  potrebn pre doruovanie _6f78ce - Copy'), 
    'KOrun')

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_In korepondenn adresa_zastupca1InaAdresa_1'))

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaSu_f321b0'), 
    '98')

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaOr_e45b31'), 
    '3')

WebUI.setText(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    '26587')
**/
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio_option_1'))

'5.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_keyboard_arrow_down_prichodZiakaRadio_10f409'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete posledn ukonen ronk zkladnej k_a84d95'), 
    '9', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_f07e4f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_2'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Aktualizova a ods_idsk-footer-extended-_552556'))

'6.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-1-1'), 
    (new Random().nextInt(4) + 29).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Slovensk jazyk a literatra, Slovensk_eedaf4'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Jazyk nrodnostnej meniny a literatra_7cfb73'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prv cudz jazyk - Anglick jazyk_selec_3c5eee'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Druh cudz jazyk - Nemeck jazyk_selec_280063'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Matematika_select-hodnotenie-1-6'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Informatika_select-hodnotenie-1-7'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prvouka, Prrodoveda, Biolgia_select-_d77479'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Fyzika_select-hodnotenie-1-9'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Chmia_select-hodnotenie-1-10'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Dejepis_select-hodnotenie-1-11'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vlastiveda, Geografia_select-hodnote_18cba2'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Obianska nuka_select-hodnotenie-1-13'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Etick vchova_select-hodnotenie-1-14'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Nboensk vchova, Nboenstvo_select-hod_81f642'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Pracovn vyuovanie, Technika_select-h_0b6ebc'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Hudobn vchova_select-hodnotenie-1-17'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vtvarn vchova_select-hodnotenie-1-18'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Telesn a portov vchova_select-hodnot_40f485'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_(nepovinn)_nav-rocnik-2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-2-1'), 
    (new Random().nextInt(4) + 29).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Slovensk jazyk a literatra, Slovensk_58cfde'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Jazyk nrodnostnej meniny a literatra_4f4d12'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prv cudz jazyk - Anglick jazyk_selec_0ba440'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Druh cudz jazyk - Nemeck jazyk_selec_3d0b8a'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Matematika_select-hodnotenie-2-6'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Informatika_select-hodnotenie-2-7'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prvouka, Prrodoveda, Biolgia_select-_753ec3'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Fyzika_select-hodnotenie-2-9'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Chmia_select-hodnotenie-2-10'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Dejepis_select-hodnotenie-2-11'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vlastiveda, Geografia_select-hodnote_9c1b7b'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Obianska nuka_select-hodnotenie-2-13'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Etick vchova_select-hodnotenie-2-14'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Nboensk vchova, Nboenstvo_select-hod_7078e1'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Pracovn vyuovanie, Technika_select-h_d7f563'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Hudobn vchova_select-hodnotenie-2-17'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vtvarn vchova_select-hodnotenie-2-18'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Telesn a portov vchova_select-hodnot_6b900c'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_(nepovinn)_nav-rocnik-3'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-3-1'), 
    (new Random().nextInt(4) + 29).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Slovensk jazyk a literatra, Slovensk_8ee786'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Jazyk nrodnostnej meniny a literatra_28ddb2'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prv cudz jazyk - Anglick jazyk_selec_b22cf4'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Druh cudz jazyk - Nemeck jazyk_selec_7b1203'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Matematika_select-hodnotenie-3-6'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Informatika_select-hodnotenie-3-7'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prvouka, Prrodoveda, Biolgia_select-_5aca68'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Fyzika_select-hodnotenie-3-9'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Chmia_select-hodnotenie-3-10'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Dejepis_select-hodnotenie-3-11'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vlastiveda, Geografia_select-hodnote_f1457e'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Obianska nuka_select-hodnotenie-3-13'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Etick vchova_select-hodnotenie-3-14'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Nboensk vchova, Nboenstvo_select-hod_c79f53'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Pracovn vyuovanie, Technika_select-h_d4b29a'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Hudobn vchova_select-hodnotenie-3-17'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vtvarn vchova_select-hodnotenie-3-18'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Telesn a portov vchova_select-hodnot_0cd9c1'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_(nepovinn)_nav-rocnik-4'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Sprvanie_select-hodnotenie-4-1'), 
    (new Random().nextInt(4) + 29).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Slovensk jazyk a literatra, Slovensk_4d1ab2'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Jazyk nrodnostnej meniny a literatra_615cd8'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prv cudz jazyk - Anglick jazyk_selec_fa63a4'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Druh cudz jazyk - Nemeck jazyk_selec_75a4c1'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Matematika_select-hodnotenie-4-6'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Informatika_select-hodnotenie-4-7'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Prvouka, Prrodoveda, Biolgia_select-_1897b0'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Fyzika_select-hodnotenie-4-9'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Chmia_select-hodnotenie-4-10'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Dejepis_select-hodnotenie-4-11'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vlastiveda, Geografia_select-hodnote_408fb1'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Obianska nuka_select-hodnotenie-4-13'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Etick vchova_select-hodnotenie-4-14'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Nboensk vchova, Nboenstvo_select-hod_adec07'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Pracovn vyuovanie, Technika_select-h_c257ec'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Hudobn vchova_select-hodnotenie-4-17'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Vtvarn vchova_select-hodnotenie-4-18'), 
    (new Random().nextInt(5) + 1).toString(), true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Telesn a portov vchova_select-hodnot_e76c58'), 
    (new Random().nextInt(5) + 1).toString(), true)

'7.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/span'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-modalNazovSutazeText'), 
    'Klokan')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalDruhSutazeSelect'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-modalUrovenSutazeSelect'), 
    '4', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_modalTypUmiestneniaRadio_option_2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_V ktorom sa iak zastnil sae_select-m_326ab7'), 
    '2024/2025', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-pridat govuk-button govuk-b_106909'))

'8.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

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

'9.krok'
WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Dokument (1).pdf  Vysvedenie z 9. ronk_f25a28'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej_btn-odoslat-prihlasku govuk-but_14d4cf'))

WebUI.click(findTestObject('Object Repository/Ditec_test/VytvoreniePrihlasky/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.waitForJQueryLoad(200)

'Prejst na preihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/a_launch_govuk-header__link'))

prihlasovanie.odhlasPouzivatela()

