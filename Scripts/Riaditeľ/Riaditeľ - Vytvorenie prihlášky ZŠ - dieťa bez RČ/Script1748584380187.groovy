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
portal.prihlasUcet('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, true)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_maDietaRCRadio'))

'Dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
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

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'Slovensko')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66_1'))

/*WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'), 
    'sloven')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66_1_2'))*/
WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3'), 
    'mong')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Veobecn informcie                      _2e0eb7'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Veobecn informcie                      _2e0eb7'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_Obec'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5'), 
    'banská by')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6'), 
    'lechk')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '2')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '87')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '62874')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Zadajte nzov predprimrneho vzdelvacie_6c01b6'), 
    'Materská škola')

'Zástupca dieťaťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'Tariel')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'Okrúhliaková')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1DatumNarodenia'), 
    '17.7.1987')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Trval pobyt dieaa_zastupca1AdresaRadio'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Trval pobyt dieaa_zastupca1AdresaRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5_6_7'), 
    'Uzbe')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _1189d1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-zastupca1InaAd_c5f597'), 
    'ul. Amira Temura 15, byt 7, 100000 Taškent, Uzbekistan')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Druh zkonn zstupca nie je znmy         _0e6a33'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Druh zkonn zstupca nie je znmy         _0e6a33'))

'Doplňujúce údaje o dieťati'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Etick_zsDPDVychovaRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/label_no'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zsDPDSkolskyKlubRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_DPDSVVPRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'test')

'Prílohy'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

'Ostatné údaje'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

Date date = new Date()

String datum = date.format('dd.MM.yyyy')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-OUDatumPodaniaPrihlasky'), 
    datum)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    'poznámka k prihláške')

'Súhrnný prehľad'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_poznmka k prihlke'), 
    'poznámka k prihláške')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Ferko Mrkva'), 
    (meno + ' ') + priezvisko)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slovensk'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_mongolsk'), 
    'mongolský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Matersk kola'), 
    'Materská škola')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Tariel Okrhliakov'), 
    'Tariel Okrúhliaková')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ul. Amira Temura 15, byt 7, 100000 Take_d24e89'), 
    'ul. Amira Temura 15, byt 7, 100000 Taškent, Uzbekistan, Uzbecká republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Druh zkonn zstupca nie je znmy'), 
    'Druhý zákonný zástupca nie je známy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Neboli nahran iadne prlohy'), 
    'Neboli nahrané žiadne prílohy.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nie'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_test'), 
    'test')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida prihlku'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaZS/Page_Prihlky a rozhodnutia  ePrihlky/span_Prihlku pre diea ste spene pridali'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

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

