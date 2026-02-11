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

Mail mail = new Mail()

Portal portal = new Portal()

def udaje = portal.nacitajNahodnyZaznam('C:\\KatalonProjects\\ePrihlasky\\Data Files\\regData.txt')

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Vitajte v pilotnej verzii portlu Elekt_a4e96e/a_Registrcia'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/h3_Vyplnenm registranho formulra'))

WebUI.waitForJQueryLoad(30)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_Meno uvete presne tak, ako je uveden _e59d5d'), 
    udaje.meno)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_Priezvisko uvete presne tak, ako je u_0ec4fe'), 
    udaje.priezvisko)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_input-email'), 'katalontest987@gmail.com')

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_input-telefon'), '+421999888777')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/button_alej'), 0)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_table-cb-all'))

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/button_alej'))

def kod = mail.getSixDigitNumberFromLastEmail('pop.gmail.com', 'pop3', 'katalontest987@gmail.com', GlobalVariable.mailHeslo)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_input-overenie-input'), 
    kod.toString())

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/button_Overi'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/span_V email katalontest987gmail.com bol sp_399808'), 
    'Váš email katalontest987@gmail.com bol úspešne overený. Dokončite registráciu, aby ste mohli pokračovať.', FailureHandling.OPTIONAL)

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_narodnostRadio'))

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_input-rodne-cislo'), 
    udaje.cislo)

WebUI.setText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_input-cislo-op'), 'EX178945')

WebUI.scrollToElement(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/button_alej_1'), 0)

if (udaje.pohlavie.equals('M')) {
    WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_pohlavieRadio'))
} else {
    WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/input_(nepovinn)_pohlavieRadioZ'))
}

WebUI.click(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/button_alej_1'))

WebUI.verifyElementNotPresent(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/span_Dolo k chybe pri komunikcii so servero_27e598'), 
    60)

WebUI.waitForJQueryLoad(60)

WebUI.delay(5)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/h3_spene sme vs zaregistrovali'), 
    'Úspešne sme vás zaregistrovali', FailureHandling.OPTIONAL)

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Page_Registrcia  ePrihlky/div_Gratulujeme spene sme vs zaregistrovali_168688'), 
    'Gratulujeme! Úspešne sme vás zaregistrovali do systému. Na váš email zasielame prihlasovacie údaje. Po ich obdržaní sa môžete prihlásiť.', 
    FailureHandling.OPTIONAL)

