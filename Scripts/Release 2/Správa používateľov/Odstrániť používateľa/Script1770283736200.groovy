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

prihlasovanie.prihlasRiaditela('930570706', 'uEdivOPFtSGvP7ePRyzmOg==', GlobalVariable.F2A, '910020859')

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Prihlky a rozhodnutia  ePrihlky/span'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/a_Profil koly_govuk-header__link'))

'Odstrániť riaditeľa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/button_Potvrdi_govuk-button govuk-button--s_11e03f'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'),
	'Používateľa ste úspešne odstránili.')

'Odstrániť spracovateľa'
WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/button_Potvrdi_govuk-button govuk-button--s_11e03f'))

WebUI.click(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/button_Zrui_btn-confirm govuk-button govuk-_36bc1e'))

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_Sprva koly  ePrihlky/span_check_circle_panel-text'), 
    'Používateľa ste úspešne odstránili.')

prihlasovanie.odhlasPouzivatela()

prihlasovanie.prihlasRiaditela('930593021', 'ZAUGwfApXs7u0wA8TalYuA==', GlobalVariable.F2A, '910020859')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_ePrihlky/span_Nespen prihlsenie do systmu_error-mess_fdac98'),
	'Ľutujeme, ale nemáte priradenú rolu. Ak sa ako zamestnanec školy prihlasujete do portálu, je potrebné aby vám riaditeľ školy v portáli ePrihlášky priradil rolu v rámci správy školy (správa používateľov) - bez priradenej role nie je možné spracovávať prihlášky.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_ePrihlky/h1_Registrcia_error-summary-title-login'),
	'Neúspešné prihlásenie do systému')

WebUI.refresh()

prihlasovanie.prihlasRiaditela('930593019', 'lNz7Bw/ivI+44MMBW0nO0A==', GlobalVariable.F2A, '910020859')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_ePrihlky/span_Nespen prihlsenie do systmu_error-mess_fdac98'),
	'Ľutujeme, ale nemáte priradenú rolu. Ak sa ako zamestnanec školy prihlasujete do portálu, je potrebné aby vám riaditeľ školy v portáli ePrihlášky priradil rolu v rámci správy školy (správa používateľov) - bez priradenej role nie je možné spracovávať prihlášky.')

WebUI.verifyElementText(findTestObject('Object Repository/Zak_test/Release2/SpravaPouzivatelov/OdstranitPouz/Page_ePrihlky/h1_Registrcia_error-summary-title-login'),
	'Neúspešné prihlásenie do systému')
