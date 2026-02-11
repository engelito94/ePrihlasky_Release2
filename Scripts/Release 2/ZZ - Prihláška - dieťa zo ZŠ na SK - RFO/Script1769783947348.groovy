import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
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
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

def filePath = RunConfiguration.getProjectDir()

def priloha = filePath + '/Data Files/Dokument (1).pdf'

prihlasovanie.prihlasPouzivatela('hodom47373@gopicta.com', 'CTSjUTGqz7ePh4ERSxjjPQ==', false, GlobalVariable.F2A)

'Vytvoriť prihlášku'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Moje prihlky  ePrihlky/a_iadne prihlky_btn-vytvorit-prihlasku'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/input_Prihlku mete poda od 1. oktbra do 31._8142a6'))

'Krok 1'
WebUI.click(findTestObject('Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_pridat'))

if (WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_(nepovinn)_govuk-label govuk-radios__label'), 
    'Milada Kamenická (14.5.2009)')) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_radioGroup-deti_option_0'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 2'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zmenenaPracovnaSchopnostRadio_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_S nadanm_specialneVVP_option_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mentlne postihnutie v kombinci s inm _972e2c'))

'Krok 3'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Nzov koly alebo jej adresa_fulltext-input-SS'), 
    'metodova')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Nzov koly alebo jej adresa_fulltext-_b34249'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_slovensk_pridat-do-prihlasky govuk-b_844eef'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_(nepovinn)_select-termin-prijimacej-_e6b33e'), 
    '11', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 4'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_b3430e'), 
    '+421962478632')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca2Radio_option_0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Meno'), 
    'Kamil')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Priezvisko'), 
    'Horváth')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte vo formte XXXXXXXXXX_input-za_626081'), 
    '850316/3944')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Na uveden e-mailov adresu bude zkonnm_20a95f'), 
    'hodom47373@gopicta.com')

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte telefnne slo vo formte s pred_5fd01d'), 
    '+421963215485')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

'Krok 5'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete kolu, ktor iak navtevuje_skola-input'), 
    '910020024')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete posledn ukonen ronk zkladnej k_6a0368'), 
    '9', true)

WebUI.setText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Uvete triedu, ktor iak navtevuje v tv_d8d371'), 
    '9.A')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Uvete koko rokov pln iak kolsk dochd_5aad1f'), 
    '9', true)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'))

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

WebUI.delay(1)

'Krok 9'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods_btn-dalej govuk-button go_178f87'))

WebUI.waitForJQueryLoad(20)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_kolsk rok_prihlaskaSkolskyRok'), 
    '2026/2027')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kolo prijmacieho konania_koloPrijimacie_73f495'), 
    '1. kolo')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_dietaMeno'), 
    'Milada')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_dietaPriezvisko'), 
    'Kamenická')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn priezvisko_dietaRodnePriezvisko'), 
    'Kamenická')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_dietaRodneCislo'), 
    '095514/1550')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_dietaDatumNarodenia'), 
    '14.05.2009')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Pohlavie_dietaPohlavie'), 
    'žena')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Miesto narodenia_dietaMiestonarodenia'), 
    'Rožňava')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nrodnos_dietaNarodnost'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos_dietaStatnaPrislusnost'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Materinsk jazyk_dietaMaterinskyJazyk'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Adresa trvalho pobytu_dietaAdresaTrvale_3d75f1'), 
    'Zvolen (Zvolen) 16, Zvolen (Zvolen), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zmenen pracovn schopnos_dpZmenenaPracov_8af9b9'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_pecilne vchovno-vzdelvacie potreby_dpSVVP'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Mentlne postihnutie_dpMentalnePostihnutie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div'), 
    '910020859')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1'), 
    'Gymnázium Metodova')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2'), 
    '7902J00')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_3'), 
    'gymnázium')

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
    'Petra')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupcaPriezvisko'), 
    'Horváthová')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupcaRodneCislo'), 
    '925617/5258')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupcaEmail'), 
    'hodom47373@gopicta.com')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupcaAdres_9d2f75'), 
    'Nám. SNP 16, Zvolen, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dtum narodenia_zakonnyZastupcaDatumNarodenia'), 
    '17.06.1992')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Shlas s komunikciou vhradne so zkonnm z_d82d0d'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Meno_zakonnyZastupca2Meno'), 
    'Kamil')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Priezvisko_zakonnyZastupca2Priezvisko'), 
    'Horváth')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Rodn slo_zakonnyZastupca2RodneCislo'), 
    '850316/3944')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Korepondenn adresa_zakonnyZastupca2Adre_f33ee1'), 
    'Zvolen (Zvolen) 16, Zvolen (Zvolen), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E-mail_zakonnyZastupca2Email'), 
    'hodom47373@gopicta.com')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Telefnne slo_zakonnyZastupca2Telefon'), 
    '+421963215485')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_estne vyhlasujem, e s podanm prihlky sh_b8b83c'), 
    'Áno')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prchod iaka_prichodZiakaSuhrn'), 
    'Zo ZŠ na Slovensku')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_EDUID zkladnej koly_eduidZSSuhrn'), 
    '910020024')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZoSlovenska/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nzov zkladnej koly_nazovZSSuhrn'), 
    'Základná škola s materskou školou, Soblahov 404')

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
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/a_launch_govuk-header__link'))

//nájdenie správnej prihlášky podľa mena dieťaťa
'Vymazanie prihlášky'
int cislo = 5

while (cislo <= 20) {
    String xpath = ('//div[' + cislo) + ']/div/div/div/div[2]/span'

    TestObject dynamicObj = new TestObject()

    dynamicObj.addProperty('xpath', ConditionType.EQUALS, xpath)

    if (WebUI.getText(dynamicObj) == 'Milada Kamenická') {
        break
    }
    
    cislo++
}

//vymazanie prihlášky
TestObject vymazButton = new TestObject()

vymazButton.addProperty('xpath', ConditionType.EQUALS, ('//div[' + cislo.toString()) + ']/div/div[3]/div[2]/a[2]')

WebUI.click(vymazButton)

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/PrihlaskaZZ/DietaZSMimoSR/Page_Moje prihlky  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

'Odhlásenie'
prihlasovanie.odhlasPouzivatela()

