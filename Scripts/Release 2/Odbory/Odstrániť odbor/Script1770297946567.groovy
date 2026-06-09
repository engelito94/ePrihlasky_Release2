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
import internal.GlobalVariable
import portal.Prihlasovanie

import org.openqa.selenium.Keys as Keys

Prihlasovanie prihlasovanie = new Prihlasovanie()

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910014291')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/span_cestovn ruch_odbor-stav badge green'), 
    'Skontrolovaný')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/span_Dokument (1).pdf_material-icons'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/button_Upravi_govuk-button govuk-button--se_559194'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/span_Odstrnenie odboru_material-icons-outlined'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/span_check_circle_panel-text'), 
    'Dokument bol úspešne vymazaný.')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/button_Upravi_govuk-button govuk-button--se_559194'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/button_Zrui_btn-odstranit-odbor govuk-butto_cf2116'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/div_close_idsk-card__heading'), 
    'Žiadne odbory v ročníku')

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Odbory/OdstranitOdbor/Page_Odbory a kritri  ePrihlky/button_Prida odbor_btnPridatOdbor'), 
    0)

prihlasovanie.odhlasPouzivatela()