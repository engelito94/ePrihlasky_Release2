import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
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
import org.openqa.selenium.Keys as Keys

Portal portal = new Portal()

def udaje = portal.nacitajNahodnyZaznam('C:\\KatalonProjects\\ePrihlasky\\Data Files\\regData.txt')

def meno = udaje.meno

def priezvisko = udaje.priezvisko

def pohlavie = udaje.pohlavie

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, true)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'))

WebUI.waitForJQueryLoad(30)

'1.krok\r\nPridať dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_maDietaRCRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-rodnePriezvisko'), 
    priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumNarodenia'), 
    '9.9.2021')

//vyber pohlavia
if (pohlavie.equals('M')) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_pohlavieRadio'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mu_pohlavieRadio'))
}

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

/*WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66'))

WebUI.delay(1)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos        (nepovinn)        _e10ca3'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66_1'))*/
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    'krist')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047_1'))

WebUI.delay(2)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5'), 
    'Snežn')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047_1'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '75')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '645')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '24897')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'2.Krok ZZ'
WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Peter')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Varga')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1DatumNarodenia'), 
    '4.5.1992')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Trval pobyt dieaa_zastupca1AdresaRadio'), 
    0)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Trval pobyt dieaa_zastupca1AdresaRadio'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Trval pobyt dieaa_zastupca1AdresaRadio'))

WebUI.scrollToElement(findTestObject('Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6'), 
    0)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6'), 
    'Sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _8b0943'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6_7'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6_7'), 
    'bratislav')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Vyhadajte obec po zadan prvch 3 znakov_dc5fec'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6_7_8'), 
    'azovsk')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_PS'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaSu_f321b0'), 
    '54')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaSu_f321b0'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaOr_e45b31'), 
    '63')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    '75316')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'3.krok Doplňujúce údaje'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_poldenn vchovu a vzdelvanie_msCeloden_3b2ba4'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_DPDSVVPRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'4.krok Prílohy'
WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Rozhodnutie sduPotvrdenie od veobecn_b7915e'), 
    '3', true)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_cloud_upload                           _f133fd'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'5.krok Ostatné údaje'
Date date = new Date()

String datum = date.format('dd.MM.yyyy')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-OUDatumPodaniaPrihlasky'), 
    datum)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'6.krok Súhrnný prehľad'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2024  2025'), 
    '2025 / 2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_30.05.2025'), 
    datum)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Petra Dlh'), 
    (meno + ' ') + priezvisko)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Dlh'), 
    priezvisko)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Snen 75645, 24897, Kristy (Sobrance), S_3cef0f'), 
    'Snežná ulica 75/645, 24897, Kristy (Sobrance), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Snen 75645, 24897, Kristy (Sobrance), S_3cef0f_1'), 
    'Snežná ulica 75/645, 24897, Kristy (Sobrance), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Peter Dlh'), 
    'Peter Varga')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Azovsk 5463, 75316, Bratislava (Bratisl_329f14'), 
    'Azovská 54/63, 75316, Bratislava-Devín (Bratislava IV), Slovenská republika')

'Pridať prihlášku'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida prihlku'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    (meno + ' ') + priezvisko)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'), 0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'))

WebUI.click(findTestObject('Zak_test/Filter_DenPodania'))

WebUI.scrollToElement(findTestObject('Zak_test/Filter_DenPodania_Den'), 0)

WebUI.click(findTestObject('Zak_test/Filter_DenPodania_Den'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/div_Nevalidovan voi RFO            Petra Dl_3153b9'), 
    0)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/span_KL'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Prihlky a rozhodnutia  ePrihlky/a_Odhlsi'))

