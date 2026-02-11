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
import com.kms.katalon.core.webui.keyword.internal.WebUIAbstractKeyword as WebUIAbstractKeyword
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.junit.runner.notification.Failure as Failure
import org.openqa.selenium.Keys as Keys

Portal portal = new Portal()

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet('930571647', 'kBvKxcei4AY8p2seZp2QWw==', GlobalVariable.F2A, true)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'), 0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_search'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Zak_test/Filter_DenPodania'))

WebUI.scrollToElement(findTestObject('Zak_test/Filter_DenPodania_Den'), 0)

WebUI.click(findTestObject('Zak_test/Filter_DenPodania_Den'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Prihlky a rozhodnutia  ePrihlky/button_Zobrazi'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi'))

WebUI.delay(2)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/body_Testovacie prostredie                 _17017b'))

Date date = new Date()

String datum = date.format('MM.yyyy')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/input_(nepovinn)_input-datum-podania'), 
    '01.' + datum)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/button_Uloi'))

WebUI.waitForPageLoad(5)

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/span_28.05.2025'), 
    '01.' + datum)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/textarea_(nepovinn)_textarea-poznamka-skoly'), 
    'test zmeny poznámky')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/button_Uloi_1'))

WebUI.waitForPageLoad(5)

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/span_test zmeny poznmky'), 
    'test zmeny poznámky')

'upraviť dieťa'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2'))

WebUI.waitForJQueryLoad(30)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input'), 
    'kóre')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1'), 
    'Labodov')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_Veobecn informcie                      _1aa588'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-adresaTPSupisneCislo'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-adresaTPOrientacneCislo'), 
    '46', FailureHandling.OPTIONAL)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-adresaTPPSC'), 
    '78654')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku'))

WebUI.waitForPageLoad(5)

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_krejsk'), 
    'kórejský')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_Laboreck 1846, 78654, Luenec, Slovensk _d9ea99'), 
    'Labodová ulica 89/46, 78654, Ilava, Slovenská republika')

'upraviť ZZ'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3'))

WebUI.waitForJQueryLoad(30)

//WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/body_Testovacie prostredie                 _17017b'))
WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1Email'), 
    'mail@test.sk')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    '98745')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_PrihlaskaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaOr_e45b31'), 
    '46')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2Telefon'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2Telefon'), 
    '+421555444666')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2AdresaPSC'), 
    '65421')

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_input-zastupca2AdresaOrien_3727cd'), 
    '97')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_agreementRadio'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_agreementRadio'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Uloi a sp na prihlku_1'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3'), 
    0)

WebUI.waitForPageLoad(5)

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_mailtest.sk'), 
    'mail@test.sk')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_Gemersk cesta 18, 98745, Luenec, Sloven_3d250b'), 
    'Gemerská cesta 18/46, 98745, Lučenec, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_Nmestie Svtho Egdia 4597, 65421, Poprad_fc346f'), 
    'Námestie Svätého Egídia 45/97, 65421, Poprad, Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_421555444666'), 
    '+421555444666')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_no'), 
    'áno')

'upraviť doplňujúce info a prílohy'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3_4'), 
    0)

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.END), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3_4'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_(nepovinn)_msCelodennaVychovaRadio'))

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.SPACE), FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/input_no_DPDSVVPRadio'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_alej'), 
    0)

WebUI.setText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'test inf. o dieťati')

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/select_Rozhodnutie sduPotvrdenie od veobecn_cdee2e'), 
    0)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/select_Rozhodnutie sduPotvrdenie od veobecn_cdee2e'), 
    '6', true)

//WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/div_cloud_upload                           _a8dabf'))
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/span_clear_rounded'))

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_cloud_upload                           _f133fd'), 
    'C:\\Users\\barcik\\Downloads\\obr.jpg')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Potvrdi'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Upravi prihlku  ePrihlky/button_Potvrdi'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Upravi_1_2_3_4'), 
    0)

WebUI.waitForPageLoad(5)

WebUI.waitForJQueryLoad(30)

WebUI.delay(2)

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_obr.jpg  Odporuanie veobecnho lekra pre d_538ca1'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_nie'), 
    'nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_nie_1'), 
    'nie')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/div_test inf. o dieati'), 
    'test inf. o dieťati')

WebUI.sendKeys(findTestObject('Object Repository/Zak_test/M'), Keys.chord(Keys.HOME), FailureHandling.OPTIONAL)

'odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/span_KL'))

WebUI.click(findTestObject('Object Repository/Zak_test/Riad_UpravaMS/Page_Podrobnosti prihlky  ePrihlky/a_Odhlsi'))

