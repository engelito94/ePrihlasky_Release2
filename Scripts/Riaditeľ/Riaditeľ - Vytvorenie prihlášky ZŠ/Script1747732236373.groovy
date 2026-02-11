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

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, true)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/a_addPrida prihlku'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_maDietaRCRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-rodneCislo'), 
    '215313/0155')

'1.krok Pridať dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-rodneCisloInfo'), 
    'value', '215313/0155', 0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-krstneMeno'), 
    'value', 'Alžbeta', 0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-priezvisko'), 
    'value', 'Hronová', 0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-rodnePriezvisko'), 
    'value', 'Hronová', 0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-datumNarodenia'), 
    'value', '13.03.2021', 0)

WebUI.verifyElementChecked(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Mu_pohlavieRadio'), 
    0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-miestoNarodenia'), 
    'value', 'Poprad', 0)

WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'value', 'slovenská', 0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    0)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66'))

//WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'),     'slove')
//WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ttna prslunos a jazyk                  _598e66_1'))
WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3'), 
    'slove')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4'), 
    'ilav')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2_3_4_5'), 
    'techn')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Krajina        (nepovinn)              _44b047_1'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'), 
    '89')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '90')

//WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_PS        (nepovinn)                   _972a51'))
WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '91247')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'2.krok ZZ dieťaťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
    'value', 'Emanuel', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Meno'), 
        'value', 'Elena', 0)
}

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
    'value', 'Zelený', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Priezvisko'), 
        'value', 'Hronová', 0)
}

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1DatumNarodenia'), 
    'value', '22.09.1980', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1DatumNarodenia'), 
        'value', '15.04.1985', 0)
}

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-icoZariadenia'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1CisloSchranky'), 
    'E1945657890')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Email'), 
    'mail@mail.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1Telefon'), 
    '+421999888777')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-nazovZariadenia'), 
    'zariadenie #1')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-icoZariadenia'), 
    '12345678')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca1AdresaRadio'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

'adresa 1.ZZ'
WebUI.verifyElementChecked(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca1AdresaRadio'), 
    0)

'2.ZZ'
WebUI.verifyElementChecked(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca2Radio'), 
    0)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Meno'), 
    0)

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Meno'), 
    'value', 'Elena', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Meno'), 
        'value', 'Emanuel', 0)
}

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Priezvisko'), 
    'value', 'Hronová', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2Priezvisko'), 
        'value', 'Zelený', 0)
}

if (!(WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2DatumNarodenia'), 
    'value', '15.04.1985', 0, FailureHandling.OPTIONAL))) {
    WebUI.verifyElementAttributeValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca2DatumNarodenia'), 
        'value', '22.09.1980', 0)
}

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca2AdresaRadio'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

'adresa 2.ZZ'
WebUI.verifyElementChecked(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zastupca2AdresaRadio'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_agreementRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_agreementRadio'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'3.krok Doplňujúce inf. o dieťati'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Etick_zsDPDVychovaRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDStravovanieRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDSkolskyKlubRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_DPDSVVPRadio'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'4.krok Pridať prílohy'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Rozhodnutie sduPotvrdenie od veobecn_224bbf'), 
    '1', true)

//WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                        Vybra sbor'),'C:\\Users\\barcik\\Downloads\\Dokument.pdf')
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_cloud_upload                           _f133fd'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'5.krok Ostatné údaje'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

Date date = new Date()

String datum = date.format('dd.MM.yyyy')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-OUDatumPodaniaPrihlasky'), 
    datum)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-OUPoznamkaText'), 
    'testovacia poznámka')

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'6.krok Súhrnný prehľad'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

'Podrobnosti o prihláške'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-'), 
    '-')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2024  2025'), 
    '2025 / 2026')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_20.5.2025'), 
    datum)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_testovacia poznmka'), 
    'testovacia poznámka')

'Dieťa'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Albeta Hronov'), 
    'Alžbeta Hronová')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Hronov'), 
    'Hronová')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_2153130155'), 
    '215313/0155')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_13.03.2021'), 
    '13.03.2021')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ensk'), 
    'ženské')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Poprad'), 
    'Poprad')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slovensk'), 
    'slovenská')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Slovensk republika'), 
    'Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slovensk_1'), 
    'slovenský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1'), 
    'testovacia poznámka')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Technick 8990, 91247, Ilava (Ilava), Sl_cbcc3a'), 
    'Technická 89/90, 91247, Ilava (Ilava), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Technick 8990, 91247, Ilava (Ilava), Sl_cbcc3a_1'), 
    'Technická 89/90, 91247, Ilava (Ilava), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2'), 
    '-')

'Info o škole'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Zkladn kola'), 
    'Základná škola')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Komenskho 12, 07301, Sobrance'), 
    'Komenského 12, 07301, Sobrance')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_slovensk_1_2'), 
    'slovenský')

'1.ZZ'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Informcie o zkonnom zstupcovi .1'), 
    'Informácie o zákonnom zástupcovi č.1:')

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Elena Hronov - Copy'), 
    'Emanuel Zelený', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Elena Hronov - Copy'), 
        'Elena Hronová')
}

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_15.04.1985 - Copy'), 
    '22.09.1980', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_15.04.1985 - Copy'), 
        '15.04.1985')
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_E1945657890'), 
    'E1945657890')

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nmestie Svtho Egdia 45, Poprad (Poprad)_ba3ebb - Copy'), 
    'Gemerská cesta 18, Lučenec (Lučenec), Slovenská republika', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nmestie Svtho Egdia 45, Poprad (Poprad)_ba3ebb - Copy'), 
        'Námestie Svätého Egídia 45, Poprad (Poprad), Slovenská republika')
}

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3'), 
    'mail@mail.sk', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3_4'), 
    '+421999888777', FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_zariadenie 1'), 
    'zariadenie #1')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_12345678'), 
    '12345678')

'2.ZZ'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Informcie o zkonnom zstupcovi .2'), 
    'Informácie o zákonnom zástupcovi č.2:')

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Emanuel Zelen - Copy'), 
    'Elena Hronová', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Emanuel Zelen - Copy'), 
        'Emanuel Zelený')
}

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_22.09.1980 - Copy'), 
    '15.04.1985', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_22.09.1980 - Copy'), 
        '22.09.1980')
}

if (!(WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Gemersk cesta 18, Luenec (Luenec), Slov_ef22e0 - Copy'), 
    'Námestie Svätého Egídia 45, Poprad (Poprad), Slovenská republika', FailureHandling.OPTIONAL))) {
    WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Gemersk cesta 18, Luenec (Luenec), Slov_ef22e0 - Copy'), 
        'Gemerská cesta 18, Lučenec (Lučenec), Slovenská republika')
}

WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3_4_5 - Copy'), 
    'E0005252431')

WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3_4_5_6 - Copy'), 
    '-')

WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3_4_5_6_7 - Copy'), 
    '-')

WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_no - Copy'), 'Áno')

'Prílohy'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Prlohy'), 
    'Prílohy')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_Dokument (1).pdf  Rozhodnutie sdu'), 
    0)

'Doplňujúce info'
WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Doplujce informcie o dieati'), 
    'Doplňujúce informácie o dieťati')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nboensk'), 
    'Náboženská')

WebUI.verifyElementText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nie - Copy'), 'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nie_1'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Nie_1_2'), 
    'Nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_-_1_2_3_4_5_6_7_8'), 
    '-')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida prihlku'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

'odoslať prihlášku'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida prihlku'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/span_Prihlku pre diea ste spene pridali'), 
    'Prihlášku pre dieťa ste úspešne pridali.')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    'Alžbeta Hronová')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'))

WebUI.click(findTestObject('Zak_test/Filter_DenPodania'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Zak_test/Filter_DenPodania_Den'), 0)

WebUI.click(findTestObject('Zak_test/Filter_DenPodania_Den'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/div_Prebieha validcia            Albeta Hro_cc6a15'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.HOME), FailureHandling.OPTIONAL)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/span_JN'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/a_Odhlsi'))

