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
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.JavascriptExecutor as JavascriptExecutor
import org.openqa.selenium.WebDriver as WebDriver

Portal portal = new Portal()

//portal.prihlasUcet2FA(GlobalVariable.login, GlobalVariable.heslo)
portal.prihlasUcet(GlobalVariable.login, GlobalVariable.heslo, GlobalVariable.F2A, false)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_NovaPrihlaska'), 0, FailureHandling.OPTIONAL)) {
	WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_NovaPrihlaska'))
} else {
	WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Vytvori nov prihlku'))
}



'Vybrať dieťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Martin Tlaienka_radioGroup-deti'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

//WebUI.delay(5)
WebUI.waitForJQueryLoad(30)

'Vybrať školy'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Na zkladn kolu_radioGroup-typ-skoly'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.delay(3)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/Button_VyhladajMS'))

WebUI.scrollToElement(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky-Vseobecne'), 
    0)

WebUI.click(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Prida do prihlky-Vseobecne'))

//WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Vybran koly                1'), 0)
//WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/li_Vybran koly                1'))
//WebUI.scrollToPosition(9999999, 9999999)
WebUI.delay(1)

'Zástupca dieťaťa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input__zastupca2Radio'))

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Severn 874874, 85214, Krpeany (Martin_e9a3d3'), 
    0)

'prihlaska ina adresa'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_Severn 874874, 85214, Krpeany (Martin_e9a3d3'))

WebUI.setText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input - Copy'), 
    'slove')

WebUI.scrollToElement(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input - Copy'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/vyberKrajiny'))

WebUI.setText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1 - Copy'), 
    'sen')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/vyberMesta'))

WebUI.setText(findTestObject('Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_govuk-input autocomplete-input_1_2 - Copy'), 
    'jablo')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/vyberUlice'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaSu_f321b0'), 
    '54')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaOr_e45b31'), 
    '895')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_input-zastupca1InaAdresaPSC'), 
    '09945')

WebUI.waitForJQueryLoad(30)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

'Doplňujúce informácie o dieťati'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_poldenn vchovu a vzdelvanie_msCeloden_3b2ba4'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/input_(nepovinn)_DPDSVVPRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPopisSVVText'), 
    'test')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/textarea_(nepovinn)_textarea-DPDPoznamkaText'), 
    'testovacia poznámka')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

WebUI.waitForJQueryLoad(30)

'Prílohy'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Odporuanie veobecnho lekra pre deti _5e1967'), 
    '4', true)

//WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                        Vybra sbor'))
WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                        Vybra sbor'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/select_Odporuanie veobecnho lekra pre deti _5e1967'), 
    '13', true)

WebUI.delay(1)

WebUI.uploadFileWithDragAndDrop(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/a_add                        Vybra sbor'), 
    'C:\\Users\\barcik\\Downloads\\Dokument.pdf')

WebUI.delay(3)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'), 
    0)

'Súhrnný prehľad'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_alej'))

WebUI.waitForJQueryLoad(30)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/span_Vae daje sme automaticky uloili'), 
    'Vaše údaje sme automaticky uložili.', FailureHandling.OPTIONAL)

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_estn prehlsenie'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/div_Shlas so spracvanm osobnch dajov'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Sp'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vytvorenie elektronickej prihlky  ePrihlky/button_Uloi a ods'))

WebUI.waitForJQueryLoad(30)

'Vymazanie rozpracovanej prihlášky'
WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_delete_outlineOdstrni_1_2_3'), 
    0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_delete_outlineOdstrni_1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/span_Rozpracovan prihlka bola spene vymazan'), 
    'Rozpracovaná prihláška bola úspešne vymazaná.', FailureHandling.OPTIONAL)

'Odhlásenie'
WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/span_MT'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Mj profil  ePrihlky/a_Odhlsi'))

