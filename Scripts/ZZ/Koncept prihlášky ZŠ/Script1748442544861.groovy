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
portal.prihlasUcet(GlobalVariable.login, GlobalVariable.heslo, GlobalVariable.F2A, false)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'))

'Vybrať dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_radioGroup-deti'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

'Vybrať školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_radioGroup-typ-skoly'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Button_VyhladajMS'))

WebUI.scrollToElement(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky-Vseobecne'), 
    0)

'Zástupca dieťaťa'
WebUI.click(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky-Vseobecne'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(1)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio'))

'Doplňujúce informácie o dieťati'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zsDPDVychovaRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_zsDPDStravovanieRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_no_zsDPDSkolskyKlubRadio'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_DPDSVVPRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPopisSVVText'), 
    'Testovací popis nadania')

'Pridať prílohy'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Informovan shlas zkonnho zstupcu o z_41724f'), 
    '5', true)

WebUI.delay(1)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                        Vybra sbor'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

//WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'),  0)
'Súhrnný prehľad'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(1)

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Kvetoslava Studen'), 
    'Kvetoslava Studená')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_1952120456'), 
    '195212/0456')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_ensk'), 
    'žena', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Masarykova 63874, 85214, Krpeany (Marti_ffc085'), 
    'Masarykova 63/874, 85214, Krpeľany (Martin), Slovenská republika')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Matilda Tlaienkov'), 
    'Matilda Tlačienková')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_25.02.1998'), 
    '25.02.1998')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Poznmka_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_(nepovinn)_checkmark'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods'), 
    0)

'Moje prihlášky'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods'))

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_delete_outlineOdstrni'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_delete_outlineOdstrni'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/span_Rozpracovan prihlka bola spene vymazan'), 
    'Rozpracovaná prihláška bola úspešne vymazaná.', FailureHandling.OPTIONAL)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/span_MT'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Odhlsi'))

