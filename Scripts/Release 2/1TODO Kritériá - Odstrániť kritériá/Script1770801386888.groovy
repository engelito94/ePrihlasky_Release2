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

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910007556')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/a_Odbory_govuk-header__link'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/select_(nepovinn)_select-zvolteOdborSelect'),
	'd0d9b83e-f2ad-4536-9b82-fc034e95ff0e', true)

'Zmazanie kritéria'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/span_2'))


WebUI.click(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/button_Zmaza vetko_ulozit-button'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Zak_test/Release2/Kritériá/Page_Odbory a kritri  ePrihlky/td_K1_kriterium-nazov'),
	0)

prihlasovanie.odhlasPouzivatela()