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

portal.prihlasUcet('930570810', 'ctqw/dIPXQi2uJsIdYZ0EQ==', GlobalVariable.F2A, true)

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/input_Vyhadvanie v prihlkach_fulltext-input'), 
    'Alžbeta Hronová')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'))

WebUI.click(findTestObject('Zak_test/Filter_DenPodania'))

WebUI.scrollToElement(findTestObject('Zak_test/Filter_DenPodania_Den'), 0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Zak_test/Filter_DenPodania_Den'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'))

WebUI.waitForJQueryLoad(30)

'Detail/Úprava profilu'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi'))

Date date = new Date()

String datum = date.format('MM.yyyy')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/input_(nepovinn)_input-datum-podania'), 
    '01.' + datum)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/button_Uloi'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/button_Uloi'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/span_28.05.2025'), 
    '01.' + datum)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/textarea_(nepovinn)_textarea-poznamka-skoly'), 
    'test zmeny poznámky')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/button_Uloi_1'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/span_test zmeny poznmky'), 
    'test zmeny poznámky')

'upraviť dieťa'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2'), 
    0)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'česk')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

//WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_Zadajte nzov predprimrneho vzdelvacie_6c01b6'), 0)
//WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_Zadajte nzov predprimrneho vzdelvacie_6c01b6'), 'materská škola')
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_krejsk'), 'český', 
    FailureHandling.OPTIONAL)

'upraviť ZZ'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaOr_e45b31'), 
    '456')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    '77788')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2Email'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2Email'), 
    'mail@test.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2Telefon'), 
    '+421333222111')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2AdresaOrien_3727cd'), 
    '99')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2AdresaPSC'), 
    '58264')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_no_agreementRadio'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_no_agreementRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/div_Informcie o zkonnom zstupcovi . 1      _b4315d'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku_1'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku_1'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_Nmestie Svtho Egdia 45456, 77788, Popra_125793'), 
    'Gemerská cesta 18/456, 77788, Lučenec, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_mailtest.sk'), 
    'mail@test.sk')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_Gemersk cesta 1899, 58264, Luenec, Slov_c11c2d'), 
    'Námestie Svätého Egídia 45/99, 58264, Poprad, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_nie'), 
    'nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_Neznmy dvod'), 
    'Neznámy dôvod')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_421333222111'), 
    '+421333222111')

'upraviť doplňujúce info a prílohy'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3_4'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3_4'))

WebUI.waitForJQueryLoad(30)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_zsDPDVychovaRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_zsDPDStravovanieRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_zsDPDSkolskyKlubRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_DPDSVVPRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/textarea_(nepovinn)_textarea-DPDPopisSVVText'), 
    'dyslexia')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_alej'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/select_Rozhodnutie sduPotvrdenie od veobecn_224bbf'), 
    '3', true)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_cloud_upload                           _f133fd'), 
    'C:\\Users\\barcik\\Downloads\\obr2.png')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/span_clear_rounded'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_Potvrdi'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Upravi prihlku  ePrihlky/button_Potvrdi'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_Etick'), 
    'Etická')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_no'), 
    'áno')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_no_1'), 
    'áno')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_no_1_2'), 
    'áno')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/div_dyslexia'), 
    'dyslexia')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/a_obr2.png  estn vyhlsenie zkonnho zstupcu'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.HOME), FailureHandling.OPTIONAL)

'odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/span_JN'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaZS/Page_Podrobnosti prihlky  ePrihlky/a_Odhlsi'))

