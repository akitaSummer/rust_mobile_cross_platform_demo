package com.cross.platform

import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.Toast
import com.cross.platform.RustSM4
import com.cross.platform.databinding.FragmentFirstBinding

/**
 * A simple [Fragment] subclass as the default destination in the navigation.
 */
class FirstFragment : Fragment() {

    private var _binding: FragmentFirstBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {

        _binding = FragmentFirstBinding.inflate(inflater, container, false)
        return binding.root

    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val editLinearLayout = view.findViewById<LinearLayout>(R.id.editTextLinearLayout)
        val buttonShow = view.findViewById<Button>(R.id.buttonShow)

        // Create EditText
        val editText = EditText(view.context)
        editText.setHint("Enter something")
        editText.layoutParams = LinearLayout.LayoutParams(
            ViewGroup.LayoutParams.MATCH_PARENT,
            ViewGroup.LayoutParams.WRAP_CONTENT)
        editText.setPadding(20, 20, 20, 20)

        // Add EditText to LinearLayout
        editLinearLayout?.addView(editText)

        buttonShow?.setOnClickListener { Toast.makeText(
            view.context, RustSM4().my_sm4(editText.text.toString()),
            Toast.LENGTH_LONG).show() }

    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}